import yaml
from datetime import datetime, timedelta

from azure.identity import DefaultAzureCredential
from azure.keyvault.secrets import SecretClient
from azure.storage.blob import BlobServiceClient


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


class ContextManager:
    def __init__(self, account) -> None:
        self.start_time = ""
        self.end_time = ""
        self.status = ""
        self.message = ""
        self.context = self.init_context()

    def init_context(self):
        context = {
            "credentials": {
                "access_key_id": "",
                "secret_access_value": "",
                "region": "",
            },
            "params": {
                "start_date": "",
                "end_date": "",
                "client_name": "",
                "granularity": "DAILY",
                "dimensions": ["LINKED_ACCOUNT", "SERVICE"],
                "metrics": ["BlendedCost"],
                "filters": "",
            },
        }
        return context


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
            account_url=self.account_url, credential=self.credential
        )

    def upload_blob(self, container_name, local_file_path, blob_name):
        blob_client = self.blob_service_client.get_blob_client(
            container=container_name, blob=blob_name
        )
        with open(local_file_path, "rb") as data:
            blob_client.upload_blob(data, overwrite=True)


##
container_finops = "finops"
f = open("/tmp/test.txt", "w")
f.write("Now the file has more content!")
f.close()
##
config = ConfigManager("config.yaml")
app_config = config.get_config()
key_vault_name = app_config["key-vault-name"]
storage_account_name = app_config["storage-account-name"]
storage = StorageManager(storage_account_name)
