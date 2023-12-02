from azure.storage.blob import BlobServiceClient, BlobClient, ContainerClient


def upload_to_azure_blob(
    file_path, account_name, account_key, container_name, blob_name
):
    # Set the Azure Blob Storage connection details
    connection_string = f"DefaultEndpointsProtocol=https;AccountName={account_name};AccountKey={account_key};EndpointSuffix=core.windows.net"

    # Create a BlobServiceClient using the connection string
    blob_service_client = BlobServiceClient.from_connection_string(connection_string)

    # Get a reference to the container
    container_client = blob_service_client.get_container_client(container_name)

    # Upload the file to Azure Blob Storage
    with open(file_path, "rb") as data:
        blob_client = container_client.get_blob_client(blob_name)
        blob_client.upload_blob(data)


# DefaultEndpointsProtocol=https;AccountName=finopsstorageaccount2003;AccountKey=MDPnx/y+PPsv63rnYL1kiepAtc/w196OIwu1dZHrCqBI1Kjz562Ja/iUDqdk9a2zExLWaKJGKLMn+AStEaNtfg==;EndpointSuffix=core.windows.net
# MDPnx/y+PPsv63rnYL1kiepAtc/w196OIwu1dZHrCqBI1Kjz562Ja/iUDqdk9a2zExLWaKJGKLMn+AStEaNtfg==
if __name__ == "__main__":
    # Specify the path of the local file to be uploaded
    local_file_path = "file.txt"

    # Specify the Azure Blob Storage details
    azure_account_name = "finopsstorageaccount2003"
    azure_account_key = "MDPnx/y+PPsv63rnYL1kiepAtc/w196OIwu1dZHrCqBI1Kjz562Ja/iUDqdk9a2zExLWaKJGKLMn+AStEaNtfg=="
    azure_container_name = "finopscontainer"
    azure_blob_name = "uploaded_file.txt"

    # Upload the file to Azure Blob Storage
    upload_to_azure_blob(
        local_file_path,
        azure_account_name,
        azure_account_key,
        azure_container_name,
        azure_blob_name,
    )
