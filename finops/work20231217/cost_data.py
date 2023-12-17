# pip install azure-identity

from azure.identity import DefaultAzureCredential
from core import StorageManager

TENANT_ID = ""
CLIENT_ID = ""
CLIENT_SECRET = ""

credential = DefaultAzureCredential(
    tenant_id=TENANT_ID, client_id=CLIENT_ID, client_secret=CLIENT_SECRET
)

azure_blob_storage = StorageManager(account_name="xxxxxx")
