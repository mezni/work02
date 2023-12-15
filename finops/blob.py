from azure.storage.blob import BlobServiceClient, BlobClient, ContainerClient
import os


class AzureBlobStorage:
    def __init__(self, connection_string, container_name):
        self.connection_string = connection_string
        self.container_name = container_name
        self.blob_service_client = BlobServiceClient.from_connection_string(
            connection_string
        )

    def create_container(self):
        container_client = self.blob_service_client.get_container_client(
            self.container_name
        )
        container_client.create_container()

    def upload_blob(self, local_file_path, blob_name):
        blob_client = self.blob_service_client.get_blob_client(
            container=self.container_name, blob=blob_name
        )
        with open(local_file_path, "rb") as data:
            blob_client.upload_blob(data)

    def download_blob(self, blob_name, local_file_path):
        blob_client = self.blob_service_client.get_blob_client(
            container=self.container_name, blob=blob_name
        )
        with open(local_file_path, "wb") as data:
            data.write(blob_client.download_blob().readall())


# Example usage:
connection_string = "your_storage_account_connection_string"
container_name = "your_container_name"
blob_storage = AzureBlobStorage(connection_string, container_name)

# Create container (if not exists)
blob_storage.create_container()

# Upload a blob
local_file_path = "path/to/your/local/file.txt"
blob_name = "example.txt"
blob_storage.upload_blob(local_file_path, blob_name)

# Download a blob
downloaded_file_path = "path/to/your/local/downloaded_file.txt"
blob_storage.download_blob(blob_name, downloaded_file_path)
