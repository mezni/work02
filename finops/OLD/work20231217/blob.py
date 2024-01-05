from azure.storage.blob import BlobServiceClient, BlobClient, ContainerClient

blob_service_client = BlobServiceClient.from_connection_string(
    conn_str=connection_string, credential=credential
)


class AzureBlobStorage:
    def __init__(self, connection_string):
        self.blob_service_client = BlobServiceClient.from_connection_string(
            connection_string
        )

    def create_container(self, container_name, credential):
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


# Example usage:

# Replace 'your_connection_string' with your actual connection string
connection_string = "your_connection_string"
container_name = "your_container_name"
blob_name = "example_blob.txt"
blob_data = b"Hello, Azure Blob Storage!"

# Create an instance of AzureBlobStorage
azure_blob_storage = AzureBlobStorage(connection_string)

# Create a container
azure_blob_storage.create_container(container_name)

# Upload a blob
azure_blob_storage.upload_blob(container_name, blob_name, blob_data)

# Download and print the blob content
downloaded_data = azure_blob_storage.download_blob(container_name, blob_name)
print(downloaded_data.decode("utf-8"))

# List blobs in the container
blobs_list = azure_blob_storage.list_blobs(container_name)
print("Blobs in the container:", blobs_list)

# Delete the blob
azure_blob_storage.delete_blob(container_name, blob_name)
