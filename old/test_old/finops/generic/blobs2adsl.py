from azure.storage.blob import BlobServiceClient
from azure.storage.filedatalake import DataLakeServiceClient


def copy_blob_to_adls(
    blob_account_name,
    blob_container_name,
    blob_blob_name,
    blob_account_key,
    adls_account_name,
    adls_file_system,
    adls_directory,
    adls_file_name,
    adls_account_key,
):
    # Connect to Azure Blob Storage
    blob_service_client = BlobServiceClient(
        account_url=f"https://{blob_account_name}.blob.core.windows.net",
        credential=blob_account_key,
    )
    blob_container_client = blob_service_client.get_container_client(
        blob_container_name
    )
    blob_client = blob_container_client.get_blob_client(blob_blob_name)

    # Connect to Azure Data Lake Storage Gen2
    adls_service_client = DataLakeServiceClient(
        account_url=f"https://{adls_account_name}.dfs.core.windows.net",
        credential=adls_account_key,
    )
    adls_file_system_client = adls_service_client.get_file_system_client(
        adls_file_system
    )
    adls_directory_client = adls_file_system_client.get_directory_client(adls_directory)
    adls_file_client = adls_directory_client.get_file_client(adls_file_name)

    # Download the content from the blob
    blob_content = blob_client.download_blob().readall()

    # Upload the content to the Data Lake Storage Gen2 file
    adls_file_client.create_file()
    adls_file_client.append_data(data=blob_content, offset=0, length=len(blob_content))
    adls_file_client.flush_data(len(blob_content))


if __name__ == "__main__":
    # Specify Azure Blob Storage details
    blob_account_name = "your_blob_account_name"
    blob_container_name = "your_blob_container_name"
    blob_blob_name = "your_blob_name"
    blob_account_key = "your_blob_account_key"

    # Specify Azure Data Lake Storage Gen2 details
    adls_account_name = "your_adls_account_name"
    adls_file_system = "your_adls_file_system"
    adls_directory = "your_adls_directory"
    adls_file_name = "your_adls_file_name"
    adls_account_key = "your_adls_account_key"

    # Copy the file from Azure Blob Storage to Azure Data Lake Storage Gen2
    copy_blob_to_adls(
        blob_account_name,
        blob_container_name,
        blob_blob_name,
        blob_account_key,
        adls_account_name,
        adls_file_system,
        adls_directory,
        adls_file_name,
        adls_account_key,
    )
