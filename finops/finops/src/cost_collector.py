__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-01-17"

import boto3
import os, sys, json, uuid, logging
from datetime import datetime, timedelta
from cost_core import Settings, ConfigManager, VaultManager, StorageManager


class CostAws:
    def __init__(self, config) -> None:
        self.start_time = datetime.now()
        self.end_time = ""
        self.context_id = str(uuid.uuid4())
        self.error = False
        self.message = ""
        self.config = config
        self.end_date = datetime.now().strftime("%Y-%m-%d")
        self.start_date = self.get_query_start_date()
        self.output_file_name = ""
        self.ce_client = self.create_client()

    def get_query_start_date(self):
        last_end_date = self.config["last_end_date"]
        if last_end_date == "":
            date_history = (
                datetime.utcnow() - timedelta(days=query_history_days)
            ).strftime("%Y-%m-%d")
            start_date = date_history[:-2] + "01"
        else:
            start_date = last_end_date
        if start_date[5:7] != self.end_date[5:7]:
            start_date = start_date[:-2] + "01"
        return start_date

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
        state = {
            "execution": {
                "context_id": self.context_id,
                "start_time": self.start_time.strftime("%d-%m-%Y %H:%M:%S"),
                "end_time": self.end_time.strftime("%d-%m-%Y %H:%M:%S"),
                "error": self.error,
                "message": self.message,
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
        return state

    def get_cost_data(self):
        results = []
        token = None
        if not self.error:
            try:
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
        output_dir = "/tmp"
        client_name = self.config["client_name"]
        cloud_name = self.config["cloud_name"]

        output_file_name = output_dir + "/" + self.config["cost_file_name"]

        self.output_file_name = output_file_name
        cost_data = self.get_cost_data()
        if not self.error:
            try:
                with open(self.output_file_name, "w") as file:
                    file.write(",".join(cost_record.keys()) + "\n")
                    for result_by_time in cost_data:
                        for group in result_by_time["Groups"]:
                            line = cost_record
                            period = result_by_time["TimePeriod"]["Start"]
                            account = group["Keys"][0]
                            service = group["Keys"][1]
                            amount = group["Metrics"]["UnblendedCost"]["Amount"]
                            unit = group["Metrics"]["UnblendedCost"]["Unit"]
                            estimated = result_by_time["Estimated"]
                            line["Client"] = client_name
                            line["Date"] = period
                            line["Provider"] = cloud_name
                            line["SubscriptionName"] = account
                            line["SubscriptionId"] = account
                            line["ServiceName"] = service
                            line["CostUSD"] = amount
                            line["Currency"] = unit
                            line_cost = ",".join(line.values())
                            file.write(line_cost + "\n")
            except:
                self.error = True
                self.message = "Cannot generate file"
            self.end_time = datetime.now()

        state = self.get_state()
        return state


def get_state(state_file_name):
    storage_mgr.download_blob(
        bronze_container,
        "logs/" + state_file_name,
        tmp_dir + "/" + state_file_name,
    )

    try:
        with open(tmp_dir + "/" + state_file_name) as fp:
            state = json.load(fp)
    except:
        state = {}
    return state


def get_query_dates(account):
    last_start_date = ""
    last_end_date = ""
    client_code = account["client_name"].replace(" ", "")
    state_file_prefix = (
        "state"
        + "_"
        + client_code
        + "_"
        + account["cloud_name"]
        + "_"
        + account["account_name"]
    )
    last_state_file_name = state_file_prefix + ".json"
    prev_state_file_name = state_file_prefix + "_prev.json"
    last_state = get_state(last_state_file_name)
    prev_state = get_state(prev_state_file_name)
    try:
        if not last_state["execution"]["error"]:
            last_start_date = last_state["params"]["start_date"]
            last_end_date = last_state["params"]["end_date"]
        else:
            last_start_date = prev_state["params"]["start_date"]
            last_end_date = prev_state["params"]["end_date"]
    except:
        pass
    return last_start_date, last_end_date


def get_account_config(account):
    secret_access_key = keyvault_mgr.get_secret(account["secret_access_key_name"])

    client_code = account["client_name"].replace(" ", "")
    last_start_date, last_end_date = get_query_dates(account)
    prefix = (
        +account_conf["client_code"]
        + "_"
        + account_conf["cloud_name"]
        + "_"
        + account_conf["account_name"]
    )
    config = {
        "client_name": account["client_name"],
        "client_code": client_code,
        "account_name": account["account_name"],
        "cloud_name": account["cloud_name"],
        "region": "ca-central-1",
        "access_key_id": account["access_key_id"],
        "secret_access_key": secret_access_key,
        "last_start_date": last_start_date,
        "last_end_date": last_end_date,
        "state_file_name": "state_" + prefix + ".json",
        "prev_state_file_name": "state_" + prefix + "_last.json",
        "cost_file_name": "finops_"
        + prefix
        + +"_"
        + datetime.now().strftime("%Y%m%d%H%M%S")
        + ".csv",
    }
    return config


# MAIN
cost_record = {
    "Client": "",
    "Date": "",
    "Provider": "",
    "SubscriptionName": "",
    "SubscriptionId": "",
    "ServiceName": "",
    "ServiceTier": "",
    "Resource": "",
    "ResourceId": "",
    "ResourceLocation": "",
    "ResourceType": "",
    "ResourceGroupName": "",
    "ResourceGroupId": "",
    "Product": "",
    "Meter": "",
    "Tags": "[]",
    "Cost": "",
    "CostUSD": "",
    "Currency": "",
}

query_history_days = 180
tmp_dir = "/tmp"

logger = logging.getLogger(__name__)
logger.setLevel(level=logging.INFO)
fh = logging.StreamHandler()
fh_formatter = logging.Formatter("%(asctime)s %(levelname)s - %(message)s")
fh.setFormatter(fh_formatter)
logger.addHandler(fh)

logger.info("Start")

settings, status = Settings().get_settings()
if status["error"]:
    logger.error(status["message"])
    logger.info("End")
    sys.exit(1)
bronze_container = settings["bronze_container"]
silver_container = settings["silver_container"]

storage_mgr = StorageManager(
    settings["storage_account_name"], settings["storage_account_key"]
)
keyvault_mgr = VaultManager(settings["key_vault_name"])

config = ConfigManager(settings["config_file_name"])
accounts, status = config.get_accounts()
if status["error"]:
    logger.error(status["message"])
    logger.info("End")
    sys.exit(1)

for account in accounts:
    logger.info(f"generate cost for : client=<{account['client_name']}>")
    if account["cloud_name"] == "aws":
        account_conf = get_account_config(account)
        cost_aws = CostAws(account_conf)
        state = cost_aws.generate_csv()
        if cost_aws.error:
            logger.error(cost_aws.message)
        else:
            cost_file = cost_aws.output_file_name
            storage_mgr.upload_blob(
                bronze_container, cost_file, os.path.basename(cost_file)
            )

        state_file_name = account_conf["state_file_name"]
        prev_state_file_name = account_conf["prev_state_file_name"]
        if not state["execution"]["error"]:
            storage_mgr.download_blob(
                bronze_container,
                "logs/" + state_file_name,
                tmp_dir + "/" + state_file_name,
            )
            storage_mgr.upload_blob(
                bronze_container,
                tmp_dir + "/" + state_file_name,
                "logs/" + prev_state_file_name,
            )

        with open(tmp_dir + "/" + state_file_name, "w") as fp:
            state_str = json.dumps(state, indent=4)
            print(state_str, file=fp)

        storage_mgr.upload_blob(
            bronze_container, tmp_dir + "/" + state_file_name, "logs/" + state_file_name
        )


logger.info("End")
