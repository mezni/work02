import yaml, json
from datetime import datetime, timedelta

from azure.identity import DefaultAzureCredential
from azure.keyvault.secrets import SecretClient
from azure.storage.blob import BlobServiceClient

__author__ = "Mohamed Ali MEZNI"
__version__ = "2023-12-19"


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

    def download_blob(self, container_name, blob_name, content):
        blob_client = self.blob_service_client.get_blob_client(
            container=container_name, blob=blob_name
        )
        return blob_client.download_blob().readall()


class ContextManager:
    def __init__(self, account) -> None:
        self.start_time = datetime.now()
        self.end_time = ""
        self.status = "success"
        self.message = ""
        self.credentials = self.init_credentials(account)
        self.params = self.init_params(account)

    def set_attribute(self, attribute, value):
        setattr(self, attribute, value)

    def init_credentials(self, account):
        credentials = {
            "account_name": account["account_name"],
            "access_key_id": account["access_key_id"],
            "secret_access_key": account["secret_access_key"],
            "secret_access_value": "",  # get key vault
        }
        return credentials

    def init_params(self, account):
        params = {
            "start_date": "",
            "end_date": "",
            "client_name": account["client_name"],
            "granularity": "DAILY",
            "dimensions": ["LINKED_ACCOUNT", "SERVICE"],
            "metrics": ["BlendedCost"],
            "filters": "",
        }
        return params

    def get_context(self):
        context = {"credentials": self.credentials, "params": self.params}
        return context

    def get_state(self):
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
    #    print(account)
    context_mgr = ContextManager(account)
    context = context_mgr.get_context()
    print(context)
    # appel Cost
    context_mgr.set_attribute("end_time", datetime.now())
    state = context_mgr.get_state()
    content = json.dumps(state, indent=2)
    storage.upload_blob(container_finops, content, "logs/state.json")
