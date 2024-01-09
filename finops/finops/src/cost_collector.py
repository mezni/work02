import boto3
import uuid
from datetime import datetime, timedelta


class CostAws:
    def __init__(self, config) -> None:
        self.start_time = datetime.now()
        self.end_time = ""
        self.context_id = str(uuid.uuid4())
        self.output_file_name = ""
        self.config = config
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
            return None

    def get_context(self):
        context = {
            "context_id": self.context_id,
            "start_time": self.start_time,
            "end_time": self.end_time,
        }
        return context

    def get_query_dates(self):
        last_start_date = self.config["last_start_date"]
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
        return start_date, end_date

    def get_cost_data(self):
        status = {"error": False, "message": ""}
        results = []
        token = None
        if not self.ce_client:
            status["error"] = True
            status["message"] = "Cannot get AWS client"
        else:
            try:
                start_date, end_date = self.get_query_dates()
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
                        **kwargs
                    )
                    results += data["ResultsByTime"]
                    token = data.get("NextPageToken")
                    if not token:
                        break
            except:
                status["error"] = True
                status["message"] = "Cannot generate AWS data"
        return results, status

    def generate_csv(self):
        status = {"error": False, "message": ""}
        columns = [
            "periode",
            "client",
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
            + client_code
            + "_"
            + cloud_name
            + "_"
            + account_name
            + "_"
            + datetime.now().strftime("%Y%m%d%H%M%S")
            + ".csv"
        )
        self.output_file_name = output_file_name
        cost_data, s = self.get_cost_data()

        if s["error"] == []:
            status["error"] = True
            status["message"] = s["message"]
        else:
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


account_cfg = {
    "client_name": "test",
    "client_code": "test",
    "account_name": "test",
    "cloud_name": "aws",
    "region": "ca-central-1",
    "access_key_id": "",
    "secret_access_key": "",
    "last_start_date": "",
    "last_end_date": "",
}

# MAIN
query_history_days = 180
cost_aws = CostAws(account_cfg)
cost_aws.generate_csv()
