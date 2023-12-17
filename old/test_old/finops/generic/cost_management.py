import requests
from azure.identity import DefaultAzureCredential

# Set the Azure subscription ID and resource group name
subscription_id = "your_subscription_id"
resource_group_name = "your_resource_group_name"

# Set the Azure Cost Management REST API endpoint
cost_management_endpoint = f"https://management.azure.com/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.CostManagement/query?api-version=2021-11-01"

# Set the Azure AD tenant ID, client ID, and client secret
tenant_id = "your_tenant_id"
client_id = "your_client_id"
client_secret = "your_client_secret"

# Acquire a token for authentication using Azure Identity
credential = DefaultAzureCredential(
    authority="https://login.microsoftonline.com/" + tenant_id
)
access_token = credential.get_token("https://management.azure.com/.default").token

# Set the headers for the REST API request
headers = {
    "Authorization": "Bearer " + access_token,
    "Content-Type": "application/json",
}

# Define the query to retrieve cost data (example: get cost by service)
query = {
    "type": "ActualCost",
    "timeframe": "MonthToDate",
    "dataset": {
        "granularity": "Daily",
        "filter": {
            "and": [
                {
                    "dimensions": {
                        "name": "ResourceGroupName",
                        "operator": "In",
                        "values": [resource_group_name],
                    },
                }
            ]
        },
        "aggregation": {
            "totalCost": {
                "name": "PreTaxCost",
                "function": "Sum",
            },
        },
    },
}

# Make the REST API request to Azure Cost Management
response = requests.post(cost_management_endpoint, json=query, headers=headers)

# Check if the request was successful (HTTP status code 200)
if response.status_code == 200:
    cost_data = response.json()
    print("Cost data:", cost_data)
else:
    print("Error:", response.status_code, response.text)
