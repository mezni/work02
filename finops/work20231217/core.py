# pip install azure-storage-blob
from azure.storage.blob import BlobServiceClient


class StorageManager:
    def __init__(self, account_name, credential) -> None:
        self.account_name = account_name
        self.credential = credential
        self.connection_string = f"DefaultEndpointsProtocol=https;AccountName={self.account_name};EndpointSuffix=core.windows.net"
        self.blob_service_client = BlobServiceClient.from_connection_string(
            conn_str=self.connection_string, credential=self.credential
        )

    def create_container(self, container_name):
        container_client = self.blob_service_client.get_container_client(container_name)
        container_client.create_container()

    def upload_blob(self, container_name, blob_name, data):
        container_client = self.blob_service_client.get_container_client(container_name)
        blob_client = container_client.get_blob_client(blob_name)
        blob_client.upload_blob(data)

    def download_blob(self, container_name, blob_name):
        container_client = self.blob_service_client.get_container_client(container_name)
        blob_client = container_client.get_blob_client(blob_name)
        blob_data = blob_client.download_blob()
        return blob_data.readall()

    def list_blobs(self, container_name):
        container_client = self.blob_service_client.get_container_client(container_name)
        blobs = container_client.list_blobs()
        return [blob.name for blob in blobs]

    def delete_blob(self, container_name, blob_name):
        container_client = self.blob_service_client.get_container_client(container_name)
        blob_client = container_client.get_blob_client(blob_name)
        blob_client.delete_blob()


class SecretManager:
    def __init__(self) -> None:
        pass
