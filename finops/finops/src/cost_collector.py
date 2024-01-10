__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-01-10"

import boto3
import uuid, logging, sys
from datetime import datetime, timedelta
from cost_core import Settings, ConfigManager, VaultManager


class CostAws:
    def __init__(self, config) -> None:
        self.start_time = datetime.now()
        self.end_time = ""
        self.context_id = str(uuid.uuid4())
        self.error = False
        self.message = ""
        self.config = config
        self.start_date = ""
        self.end_date = ""
        self.output_file_name = ""
        self.ce_client = self.create_client()

    def create_client(self):
        try:
            return boto3.client(
                "ce",
                aws_access_key_id=self.config["access_key_id"],
                aws_secret_access_key=self.config["secret_access_key"],
                region_name=self.config["region"],
            )
        except Exception as e:
            self.error = True
            self.message = "Cannot create AWS client"
            self.end_time = datetime.now()
            return None

    def get_state(self):
        context = {
            "execution": {
                "context_id": self.context_id,
                "start_time": self.start_time.strftime("%d-%m-%Y %H:%M:%S"),
                "end_time": self.end_time.strftime("%d-%m-%Y %H:%M:%S"),
                "error": self.error,
                "error": self.message,
                "output_file": self.output_file_name,
            },
            "params": {
                "start_date": self.start_date,
                "end_date": self.end_date,
                "granularity": "DAILY",
                "dimensions": ["LINKED_ACCOUNT", "SERVICE"],
                "filter": "",
            },
        }
        return context

    def get_query_dates(self):
        last_end_date = self.config["last_end_date"]
        end_date = datetime.now().strftime("%Y-%m-%d")
        if last_end_date == "":
            date_history = (
                datetime.utcnow() - timedelta(days=query_history_days)
            ).strftime("%Y-%m-%d")
            start_date = date_history[:-2] + "01"
        else:
            start_date = last_end_date
        if start_date[5:7] != end_date[5:7]:
            start_date = start_date[:-2] + "01"
        self.start_date = start_date
        self.end_date = end_date

    def get_cost_data(self):
        results = []
        token = None
        if not self.error:
            try:
                self.get_query_dates()
                start_date = self.start_date
                end_date = self.end_date
                while True:
                    if token:
                        kwargs = {"NextPageToken": token}
                    else:
                        kwargs = {}
                    data = self.ce_client.get_cost_and_usage(
                        TimePeriod={"Start": start_date, "End": end_date},
                        Granularity="DAILY",
                        Metrics=["UnblendedCost"],
                        GroupBy=[
                            {"Type": "DIMENSION", "Key": "LINKED_ACCOUNT"},
                            {"Type": "DIMENSION", "Key": "SERVICE"},
                        ],
                        **kwargs,
                    )
                    results += data["ResultsByTime"]
                    token = data.get("NextPageToken")
                    if not token:
                        break
            except:
                self.error = True
                self.message = "Cannot generate AWS data"
                self.end_time = datetime.now()
        return results

    def generate_csv(self):
        columns = [
            "periode",
            "client",
            "cloud",
            "compte",
            "service",
            "cout",
            "devise",
            "estimation",
        ]

        output_dir = "/tmp"
        client_code = self.config["client_code"]
        client_name = self.config["client_name"]
        cloud_name = self.config["cloud_name"]
        account_name = self.config["account_name"]

        output_file_name = (
            output_dir
            + "/"
            + "finops_"
            + client_name
            + "_"
            + cloud_name
            + "_"
            + account_name
            + "_"
            + datetime.now().strftime("%Y%m%d%H%M%S")
            + ".csv"
        )
        self.output_file_name = output_file_name

        cost_data = self.get_cost_data()
        if not self.error:
            try:
                with open(self.output_file_name, "w") as file:
                    file.write(",".join(columns) + "\n")
                    for result_by_time in cost_data:
                        for group in result_by_time["Groups"]:
                            period = result_by_time["TimePeriod"]["Start"]
                            account = group["Keys"][0]
                            service = group["Keys"][1]
                            amount = group["Metrics"]["UnblendedCost"]["Amount"]
                            unit = group["Metrics"]["UnblendedCost"]["Unit"]
                            estimated = result_by_time["Estimated"]
                            line = (
                                period
                                + ","
                                + client_code
                                + ","
                                + account
                                + ","
                                + service
                                + ","
                                + amount
                                + ","
                                + unit
                                + ","
                                + str(estimated)
                            )
                            file.write(line + "\n")
            except:
                self.error = True
                self.message = "Cannot generate file"
            self.end_time = datetime.now()

        state = self.get_state()
        return state


# MAIN

logging.basicConfig(
    format="%(asctime)s - %(levelname)s - %(message)s", level=logging.INFO
)
logging.info("Start")

query_history_days = 180

settings, status = Settings().get_settings()
if status["error"]:
    logging.error(status["message"])
    logging.info("End")
    sys.exit(1)

# keyvault_mgr = VaultManager(settings["key_vault_name"])

config = ConfigManager(settings["config_file_name"])
accounts, status = config.get_accounts()
if status["error"]:
    logging.error(status["message"])
    logging.info("End")
    sys.exit(1)

for account in accounts:
    logging.info(f"generate cost for : client=<{account['client_name']}>")
    if account["cloud_name"] == "aws":
        secret_access_key = None
        #        secret_access_key = keyvault_mgr.create_secret_client(
        #            account["secret_access_key_name"]
        #        )
        account_cfg = {
            "client_name": account["client_name"],
            "client_code": account["client_name"].replace(" ", ""),
            "account_name": account["account_name"],
            "cloud_name": account["cloud_name"],
            "region": "ca-central-1",
            "access_key_id": account["access_key_id"],
            "secret_access_key": secret_access_key,
            "last_start_date": "",
            "last_end_date": "",
        }
        cost_aws = CostAws(account_cfg)
        state = cost_aws.generate_csv()

logging.info("End")
