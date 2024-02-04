__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-02-05"

import yaml
from datetime import datetime
from pydantic_settings import BaseSettings, SettingsConfigDict
from azure.identity import DefaultAzureCredential
from azure.keyvault.secrets import SecretClient
from azure.storage.blob import BlobServiceClient

env_file = "env"


class Settings(BaseSettings):
    KEY_VAULT_NAME: str
    STORAGE_ACCOUNT_NAME: str
    STORAGE_ACCOUNT_KEY: str
    WORK_CONTAINER: str
    BRONZE_CONTAINER: str
    SILVER_CONTAINER: str

    model_config = SettingsConfigDict(env_file=env_file)

    def get_settings(self) -> dict:
        try:
            settings = {
                "key_vault_name": self.KEY_VAULT_NAME,
                "storage_account_name": self.STORAGE_ACCOUNT_NAME,
                "storage_account_key": self.STORAGE_ACCOUNT_KEY,
                "work_container": self.WORK_CONTAINER,
                "bronze_container": self.BRONZE_CONTAINER,
                "silver_container": self.SILVER_CONTAINER,
            }
        except:
            settings = {}
        return settings


class ConfigManager:
    def __init__(self, config_file) -> None:
        self.config_file = config_file
        self.config_data = self.load_config()

    def load_config(self) -> dict:
        try:
            with open(self.config_file, "r") as file:
                data = yaml.safe_load(file)
        except:
            data = {}
        return data

    def get_accounts(self) -> list:
        accounts = []
        clients_data = self.config_data["clients"]

        for cli in clients_data:
            client = cli.get("name", "")
            for acc in cli["accounts"]:
                account = acc.get("name", "")
                cloud = acc.get("cloud", "")
                credentials = acc.get("credentials", {})
                client_code = client.replace(" ", "")
                account_code = account.replace("-", "_")
                file_prefix = client_code + "_" + account_code
                last_state_file_name = "state" + "_" + file_prefix + ".json"
                prev_state_file_name = "state" + "_" + file_prefix + "_prev.json"
                ts = datetime.now().strftime("%Y%m%d%H%M%S")
                cost_file_name = "finops_" + file_prefix + "_" + ts + ".csv"

                account = {
                    "client": client,
                    "account": account,
                    "cloud": cloud,
                    "credentials": credentials,
                    "last_state_file_name": last_state_file_name,
                    "prev_state_file_name": prev_state_file_name,
                    "cost_file_name": cost_file_name,
                }
                accounts.append(account)
        return accounts


class StorageManager:
    def __init__(self, account_name, account_key) -> None:
        self.account_name = account_name
        self.account_key = account_key  # DefaultAzureCredential()
        self.blob_service_client = self.create_blob_client()

    def create_blob_client(self):
        account_url = f"https://{self.account_name}.blob.core.windows.net"
        blob_service_client = BlobServiceClient(
            account_url=account_url, credential=self.account_key
        )
        return blob_service_client

    def download_blob(self, container_name, blob_name, file_name):
        container_client = self.blob_service_client.get_container_client(
            container=container_name
        )
        try:
            blob_client = container_client.get_blob_client(blob_name)
            with open(file_name, "wb") as local_file:
                blob_data = blob_client.download_blob()
                local_file.write(blob_data.readall())
        except:
            pass

    def upload_blob(self, container_name, file_name, blob_name):
        container_client = self.blob_service_client.get_container_client(
            container=container_name
        )
        try:
            with open(file_name, "rb") as data:
                container_client.upload_blob(name=blob_name, data=data, overwrite=True)
        except:
            pass

    def list_blobs(self, container_name):
        blobs = []
        container_client = self.blob_service_client.get_container_client(
            container=container_name
        )
        result = container_client.list_blobs()
        for r in result:
            blobs.append(r.name)
        return blobs

    def delete_blob(self, container_name, blob_name):
        container_client = self.blob_service_client.get_container_client(
            container=container_name
        )
        blob_client = container_client.get_blob_client(blob_name)
        blob_client.delete_blob()


class VaultManager:
    def __init__(self, key_vault_name) -> None:
        keyvault_url = f"https://{key_vault_name}.vault.azure.net"
        self.status = {}
        self.keyvault_url = keyvault_url
        self.credentials = self.get_credentials()
        self.secret_client = self.create_secret_client()

    def get_credentials(self):
        try:
            credentials = DefaultAzureCredential()
        except:
            credentials = None
        return credentials

    def create_secret_client(self):
        return SecretClient(vault_url=self.keyvault_url, credential=self.credentials)

    def get_secret(self, secret_access_key):
        try:
            secret = self.secret_client.get_secret(secret_access_key).value
        except Exception as e:
            secret = ""
        return secret
