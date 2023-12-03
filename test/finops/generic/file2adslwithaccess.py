from azure.storage.filedatalake import DataLakeServiceClient
from azure.storage.filedatalake._shared._shared_access_signature import (
    FileSasPermissions,
)
from azure.storage.blob import generate_blob_sas, BlobSasPermissions
from azure.storage.blob._shared_access_signature import BlobSharedAccessSignature
from datetime import datetime, timedelta


def upload_to_adls(
    file_path,
    adls_account_name,
    adls_file_system,
    adls_directory,
    adls_file_name,
    storage_account_key,
):
    # Set the Azure Data Lake Storage connection details
    adls_url = f"https://{adls_account_name}.dfs.core.windows.net"

    # Generate a SAS token for authentication
    sas_token = generate_sas_token(
        adls_account_name, adls_file_system, storage_account_key
    )

    # Create a DataLakeServiceClient using the connection details and SAS token
    service_client = DataLakeServiceClient(account_url=adls_url, credential=sas_token)

    # Get the file system client
    file_system_client = service_client.get_file_system_client(
        file_system=adls_file_system
    )

    # Get the directory client
    directory_client = file_system_client.get_directory_client(directory=adls_directory)

    # Get the file client
    file_client = directory_client.get_file_client(file=adls_file_name)

    # Upload the file to Azure Data Lake Storage
    with open(file_path, "rb") as file:
        file_client.create_file()
        file_client.append_data(data=file.read(), offset=0, length=len(file.read()))
        file_client.flush_data(len(file.read()))


def generate_sas_token(account_name, container_name, account_key):
    expiry = datetime.utcnow() + timedelta(
        hours=1
    )  # Adjust the expiration time as needed
    sas_permissions = BlobSasPermissions(read=True, write=True, delete=True, list=True)

    sas_token = BlobSharedAccessSignature(account_name, account_key)
    return sas_token.generate_blob(
        account_name,
        container_name,
        account_name + "/" + container_name,
        expiry,
        sas_permissions,
    )


if __name__ == "__main__":
    # Specify the path of the local file to be uploaded
    local_file_path = "path/to/local/file.txt"

    # Specify the Azure Data Lake Storage details
    azure_adls_account_name = "your_adls_account_name"
    azure_adls_file_system = "your_adls_file_system"
    azure_adls_directory = "your_adls_directory"
    azure_adls_file_name = "uploaded_file.txt"
    azure_storage_account_key = "your_storage_account_key"

    # Upload the file to Azure Data Lake Storage
    upload_to_adls(
        local_file_path,
        azure_adls_account_name,
        azure_adls_file_system,
        azure_adls_directory,
        azure_adls_file_name,
        azure_storage_account_key,
    )
