# pip install azure-identity

from azure.identity import DefaultAzureCredential
from finops.work20231217.core_01 import StorageManager

TENANT_ID = ""
CLIENT_ID = ""
CLIENT_SECRET = ""

finops_credential = DefaultAzureCredential(
    tenant_id=TENANT_ID, client_id=CLIENT_ID, client_secret=CLIENT_SECRET
)

finops_blob_storage = StorageManager(
    account_name="xxxxxx", credential=finops_credential
)
