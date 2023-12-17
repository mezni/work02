** azure create client_id and client_secret

To create a client ID and client secret in Azure, you typically need to register an application in the Azure Active Directory (Azure AD) associated with your Azure subscription. Here are the general steps to create a client ID and client secret:

    Sign in to the Azure portal:
        Go to Azure portal.
        Sign in with your Azure account.

    Navigate to Azure AD:
        In the left-hand navigation pane, click on "Azure Active Directory."

    Register an application:
        Click on "App registrations" and then click on "New registration."
        Provide a name for your application, choose the appropriate account type, and specify the redirect URI if needed.

    Configure application settings:
        Once the application is registered, go to the "Certificates & secrets" section.
        In the "Client secrets" section, click on "New client secret."
        Provide a description, choose an expiry period, and click "Add." Make sure to copy the generated client secret value immediately as it won't be shown again.

    Get the Application (client) ID:
        In the "Overview" section, you will find the "Application (client) ID." Copy this value; it is your client ID.

    Retrieve Directory (tenant) ID:
        In the "Overview" section, find the "Directory (tenant) ID." Copy this value.

Now you have the necessary information:

    Client ID: This is the "Application (client) ID" you copied.
    Client Secret: This is the secret you generated in the "Certificates & secrets" section.
    Directory (Tenant) ID: This is the "Directory (tenant) ID" you copied.


** Grant Access to Key Vault:

Now, you need to give the application access to the Azure Key Vault:

    Go to the Azure Key Vault in the Azure portal.

    In the Key Vault's menu, select "Access policies."

    Click on the "Add Access Policy" button.

    In the "Secret permissions" section, check the permissions you want to grant (e.g., Get, List, Set, etc.).

    In the "Select principal" section, search for and select your registered application.

    Click on "Add" to save the access policy.


** Grant Access to Storage Account:

    Go to the Azure portal.

    Navigate to your Storage account.

    In the left navigation, click on "Access control (IAM)."

    Click on "+ Add a role assignment."

    Select a role (e.g., Storage Blob Data Contributor) that suits your needs.

    In the "Assign access to" field, search for and select your registered application.

    Click "Save" to add the role assignment.



** access 
from azure.identity import DefaultAzureCredential
from azure.storage.blob import BlobServiceClient

# Azure AD details
tenant_id = "<your-tenant-id>"
client_id = "<your-client-id>"
client_secret = "<your-client-secret>"

# Storage account details
account_name = "<your-storage-account-name>"
container_name = "<your-container-name>"

# Use DefaultAzureCredential to authenticate
credential = DefaultAzureCredential(tenant_id=tenant_id, client_id=client_id, client_secret=client_secret)

# Construct the Storage account connection string
connection_string = f"DefaultEndpointsProtocol=https;AccountName={account_name};EndpointSuffix=core.windows.net"

# Create a BlobServiceClient
blob_service_client = BlobServiceClient.from_connection_string(conn_str=connection_string, credential=credential)

# Access the container and list blobs
container_client = blob_service_client.get_container_client(container_name)
blobs = container_client.list_blobs()

for blob in blobs:
    print(blob.name)



os.environ["AZURE_TENANT_ID"] = "<your-tenant-id>"
os.environ["AZURE_CLIENT_ID"] = "<your-client-id>"
os.environ["AZURE_CLIENT_SECRET"] = "<your-client-secret>"
