from azure.storage.blob import BlobServiceClient, BlobClient, ContainerClient
from azure.identity import DefaultAzureCredential


class StorageManager:
    def __init__(self, account_name, account_key):
        self.account_name = account_name
        self.account_key = account_key
        self.credential = DefaultAzureCredential()
        self.blob_service_client = self.create_blob_service_client()

    def create_blob_service_client(self):
        connection_string = f"DefaultEndpointsProtocol=https;AccountName={self.account_name};AccountKey={self.account_name};EndpointSuffix=core.windows.net"
        return BlobServiceClient.from_connection_string(
            connection_string, credential=self.credential
        )

    #    def create_container(self):
    #        container_client = self.blob_service_client.get_container_client(self.container_name)
    #        container_client.create_container()

    def upload_blob(self, container_name, local_file_path, blob_name):
        blob_client = self.blob_service_client.get_blob_client(
            container=container_name, blob=blob_name
        )
        with open(local_file_path, "rb") as data:
            blob_client.upload_blob(data, overwrite=True)

    def download_blob(self, container_name, blob_name, local_file_path):
        blob_client = self.blob_service_client.get_blob_client(
            container=container_name, blob=blob_name
        )
        with open(local_file_path, "wb") as data:
            data.write(blob_client.download_blob().readall())

    def delete_blob(self, container_name, blob_name):
        blob_client = self.blob_service_client.get_blob_client(
            container=container_name, blob=blob_name
        )
        blob_client.delete_blob()


storage_account_name = "your_storage_account_name"
storage_account_key = ""
storage = StorageManager(storage_account_name, storage_account_key)


blob_storage = AzureBlobStorage(account_name, container_name)

# Create the container (if it doesn't exist)
blob_storage.create_container()

# Upload a blob
local_file_path = "path/to/local/file.txt"
blob_name = "file.txt"
blob_storage.upload_blob(local_file_path, blob_name)

# Download the blob
downloaded_file_path = "path/to/local/downloaded_file.txt"
blob_storage.download_blob(blob_name, downloaded_file_path)

# Delete the blob
blob_storage.delete_blob(blob_name)
