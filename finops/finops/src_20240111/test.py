import yaml
from pydantic_settings import BaseSettings, SettingsConfigDict
from azure.identity import DefaultAzureCredential
from azure.keyvault.secrets import SecretClient
from azure.storage.blob import BlobServiceClient, BlobClient, ContainerClient


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

    #    def upload_content(self, container_name, content, blob_name):
    #        blob_client = self.blob_service_client.get_blob_client(
    #            container=container_name, blob=blob_name
    #        )
    #        blob_client.upload_blob(content, overwrite=True)

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
                container_client.upload_blob(name=blob_name, data=data)
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


bronze_container = "xfinops-work"
STORAGE_ACCOUNT_NAME = "datalakeopportunitedev"
STORAGE_ACCOUNT_KEY = "sabBQX0qlN+NWRPvunmgaiiAFPWYHGiE4C3F40iv5m3WyuFoEg5NhFzRYPQWna7LujhRsBHNEZZSNyN9VeDFwQ=="
storage_mgr = StorageManager(STORAGE_ACCOUNT_NAME, STORAGE_ACCOUNT_KEY)
storage_mgr.upload_blob(bronze_container, "cost_core.py", "cost_core.py")
x = storage_mgr.list_blobs(bronze_container)
storage_mgr.download_blob(bronze_container, "cost_core.py", "xxx.py")
print(x)
