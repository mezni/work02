import yaml, json
from datetime import datetime, timedelta
import boto3
from azure.identity import DefaultAzureCredential
from azure.keyvault.secrets import SecretClient
from azure.storage.blob import BlobServiceClient
from azure.core.exceptions import ResourceNotFoundError

__author__ = "Mohamed Ali MEZNI"
__version__ = "2023-12-20"


class ConfigManager:
    def __init__(self, config_file) -> None:
        self.config_file = config_file
        self.config_data = self.load_config()
        self.accounts = self.get_accounts()

    def load_config(self) -> dict:
        try:
            with open(self.config_file, "r") as file:
                data = yaml.safe_load(file)
        except:
            data = {}
        return data

    def get_config(self) -> list:
        return self.config_data["app"]

    def get_accounts(self) -> list:
        accounts = []
        try:
            for acc in self.config_data["clients"]:
                account = {
                    "client_name": acc["client_name"],
                    "client_code": acc["client_code"],
                    "cloud_name": acc["cloud_name"],
                    "account_name": acc["account_name"],
                    "access_key_id": acc["access_key_id"],
                    "secret_access_key": acc["secret_access_key"],
                }
                accounts.append(account)
        except:
            pass
        return accounts


class VaultManager:
    def __init__(self, key_vault_name) -> None:
        keyvault_url = f"https://{key_vault_name}.vault.azure.net"
        self.status = ""
        self.message = ""
        self.keyvault_url = keyvault_url
        self.credentials = DefaultAzureCredential()
        self.secret_client = self.create_secret_client()

    def create_secret_client(self):
        return SecretClient(vault_url=self.keyvault_url, credential=self.credentials)

    def get_secret(self, secret_name):
        try:
            secret = self.secret_client.get_secret(secret_name)
            return secret.value
        except Exception as e:
            self.status = "failed"
            self.message = e
            return None


class StorageManager:
    def __init__(self, account_name) -> None:
        account_url = f"https://{account_name}.blob.core.windows.net"
        self.account_name = account_name
        self.status = ""
        self.message = ""
        self.account_url = account_url
        self.credentials = DefaultAzureCredential()
        self.blob_service_client = self.create_blob_client()

    def create_blob_client(self):
        return BlobServiceClient(
            account_url=self.account_url, credential=self.credentials
        )

    def upload_blob(self, container_name, content, blob_name):
        blob_client = self.blob_service_client.get_blob_client(
            container=container_name, blob=blob_name
        )
        blob_client.upload_blob(content, overwrite=True)

    def download_blob(self, container_name, blob_name):
        blob_client = self.blob_service_client.get_blob_client(
            container=container_name, blob=blob_name
        )
        return blob_client.download_blob().readall()


class CostAws:
    def __init__(self, context) -> None:
        self.status = None
        self.message = None
        self.credentials = context.get("credentials")
        self.params = context.get("params")
        self.ce_client = self.create_ce_client()

    def create_ce_client(self):
        try:
            return boto3.client(
                "ce",
                aws_access_key_id=self.credentials.get("access_key_id"),
                aws_secret_access_key=self.credentials.get("secret_access_value"),
                region_name=self.credentials.get("region"),
            )
        except Exception as e:
            self.status = "failed"
            self.message = e
            return None

    def get_cost(self):
        results = []

        start_date = self.params.get("start_date")
        end_date = self.params.get("end_date")
        granularity = self.params.get("granularity")
        dimensions = [
            {"Type": "DIMENSION", "Key": d} for d in self.params.get("dimensions")
        ]
        metrics = self.params.get("metrics")
        filters = self.params.get("filters")

        results = self.ce_client.get_cost_and_usage(
            TimePeriod={"Start": start_date, "End": end_date},
            Granularity=granularity,
            Metrics=metrics,
            GroupBy=dimensions,
        )

        return results


class ContextManager:
    def __init__(self, account, key_vault, storage) -> None:
        self.start_time = datetime.now()
        self.end_time = ""
        self.status = "success"
        self.message = ""
        self.credentials = self.init_credentials(account)
        self.variables = self.init_variables(account)
        self.params = self.init_params(account)

    def set_attribute(self, attribute, value):
        setattr(self, attribute, value)

    def init_variables(self, account):
        variables = {
            "client_name": account["client_name"],
            "client_code": account["client_code"],
            "account_name": account["account_name"],
            "file_name": "finops"
            + "_"
            + account["client_code"]
            + "_"
            + account["account_name"]
            + "_"
            + self.start_time.strftime("%Y%m%d")
            + "."
            + "csv",
            "state_last_name": "last_state_" + account["account_name"] + ".json",
            "state_prev_name": "prev_state_" + account["account_name"] + ".json",
        }
        return variables

    def init_credentials(self, account):
        try:
            secret_access_value = key_vault.get_secret(account["secret_access_key"])
        except Exception as e:
            secret_access_value = ""

        credentials = {
            "account_name": account["account_name"],
            "access_key_id": account["access_key_id"],
            "secret_access_key": account["secret_access_key"],
            "secret_access_value": secret_access_value,
        }
        return credentials

    def init_params(self, account):
        state_last_file = self.variables["state_last_name"]
        state_prev_file = self.variables["state_prev_name"]
        try:
            state_last_content = storage.download_blob(
                container_finops, state_last_file
            )
        except:
            state_last_content = {}
        try:
            state_prev_content = storage.download_blob(
                container_finops, state_prev_file
            )
        except:
            state_prev_content = {}

        start_date, end_date = self.get_dates(state_last_content, state_prev_content)
        params = {
            "start_date": start_date,
            "end_date": end_date,
            "client_name": self.variables["client_name"],
            "granularity": "DAILY",
            "dimensions": ["LINKED_ACCOUNT", "SERVICE"],
            "metrics": ["UnblendedCost"],
            "filters": "",
        }
        return params

    def get_dates(self, state_last_content, state_prev_content):
        start_date = ""
        end_date = self.start_time.strftime("%Y-%m-%d")
        if not state_last_content and not state_prev_content:
            start_date_dt = self.start_time - timedelta(days=335)
            start_date = start_date_dt.strftime("%Y-%m") + "-01"
        return start_date, end_date

    def get_context(self):
        context = {"credentials": self.credentials, "params": self.params}
        return context

    def write_state(self):
        # write state and manager previous state
        state = {
            "execution": {
                "start_time": self.start_time.strftime("%d-%m-%Y %H:%M:%S"),
                "end_time": self.start_time.strftime("%d-%m-%Y %H:%M:%S"),
                "status": self.status,
                "message": self.message,
            },
            "params": self.params,
        }
        return state

    def write_cost(self, cost_data):
        columns = [
            "periode",
            "client",
            "compte",
            "service",
            "cout",
            "devise",
            "estimation",
        ]
        with open("/tmp/" + self.variables["file_name"], "a") as file:
            file.write(",".join(columns) + "\n")
            for result_by_time in cost_data["ResultsByTime"]:
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
                        + self.variables["clinet_code"]
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
                        + "\n"
                    )
                    file.write(line + "\n")

        with open("/tmp/" + self.variables["file_name"], "r") as file:
            content = file.read()
            storage.upload_blob(container_bronze, content, self.variables["file_name"])
            self.end_time = datetime.now()


##
container_finops = "finops"
container_bronze = "bronze"
##
config = ConfigManager("config.yaml")
app_config = config.get_config()
key_vault_name = app_config["key-vault-name"]
storage_account_name = app_config["storage-account-name"]
key_vault = VaultManager(key_vault_name)
storage = StorageManager(storage_account_name)

accounts = config.get_accounts()
for account in accounts:
    context_mgr = ContextManager(account, key_vault, storage)
    if account["cloud_name"] == "aws":
        context = context_mgr.get_context()
        context["params"]["start_date"] = "2023-12-01"
        context["params"]["end_date"] = "2023-12-02"
        cost_data = CostAws(context).get_cost()
    context_mgr.write_cost(cost_data)
