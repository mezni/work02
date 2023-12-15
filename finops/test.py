from azure.storage.blob import BlobServiceClient, BlobClient, ContainerClient
import os


class BlobManager:
    def __init__(self, account_name, account_key):
        self.account_name = account_name
        self.account_key = account_key
        self.connection_string = "DefaultEndpointsProtocol=https;AccountName={account_name};AccountKey={accountkey};EndpointSuffix=core.windows.net"
        self.blob_service_client = BlobServiceClient.from_connection_string(
            self.connection_string
        )

    def read_blob(self, container_name, blob_name, local_file_path):
        blob_client = self.blob_service_client.get_blob_client(
            container=container_name, blob=blob_name
        )
        with open(local_file_path, "wb") as data:
            data.write(blob_client.download_blob().readall())

    def write_blob(self, container_name, blob_name, local_file_path):
        blob_client = self.blob_service_client.get_blob_client(
            container=container_name, blob=blob_name
        )
        with open(local_file_path, "rb") as data:
            blob_client.upload_blob(data)


blob_manager = BlobManager("XXXX", "YYYY")
blob_manager.read_blob("bronze", "zigzig", "./test.txt")
