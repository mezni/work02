# pip install azure-storage-file-datalake

from azure.identity import DefaultAzureCredential
from azure.storage.filedatalake import DataLakeServiceClient


def upload_to_adls(
    file_path, adls_account_name, adls_file_system, adls_directory, adls_file_name
):
    # Set the Azure Data Lake Storage connection details
    adls_url = f"https://{adls_account_name}.dfs.core.windows.net"
    credential = DefaultAzureCredential()

    # Create a DataLakeServiceClient using the connection details
    service_client = DataLakeServiceClient(account_url=adls_url, credential=credential)

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


if __name__ == "__main__":
    # Specify the path of the local file to be uploaded
    local_file_path = "path/to/local/file.txt"

    # Specify the Azure Data Lake Storage details
    azure_adls_account_name = "your_adls_account_name"
    azure_adls_file_system = "your_adls_file_system"
    azure_adls_directory = "your_adls_directory"
    azure_adls_file_name = "uploaded_file.txt"

    # Upload the file to Azure Data Lake Storage
    upload_to_adls(
        local_file_path,
        azure_adls_account_name,
        azure_adls_file_system,
        azure_adls_directory,
        azure_adls_file_name,
    )
