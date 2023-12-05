from azure.identity import DefaultAzureCredential
from azure.mgmt.resource import SubscriptionClient, ResourceManagementClient
import requests


def get_subscriptions(credential):
    subscription_client = SubscriptionClient(credential)
    subscriptions = list(subscription_client.subscriptions.list())
    return subscriptions


def get_resource_groups(credential, subscription_id):
    resource_client = ResourceManagementClient(credential, subscription_id)
    resource_groups = list(resource_client.resource_groups.list())
    return resource_groups


resource_groups = []
credential = DefaultAzureCredential()
subscriptions = get_subscriptions(credential)
for subscription in subscriptions:
    subscription_id = subscription.subscription_id
    tenant_id = subscription.tenant_id
    resource_groups = get_resource_groups(credential, subscription_id)
    for resource_group in resource_groups:
        resource_group_name = resource_group.name
        cost_management_endpoint = f"https://management.azure.com/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.CostManagement/query?api-version=2023-11-01"

        print(cost_management_endpoint)
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
        credential1 = DefaultAzureCredential(
            authority="https://login.microsoftonline.com/" + tenant_id
        )
        #        print(credential1)
        access_token = credential1.get_token(
            "https://management.azure.com/.default"
        ).token
        #        print(access_token)
        # Set the headers for the REST API request
        headers = {
            "Authorization": "Bearer " + access_token,
            "Content-Type": "application/json",
        }

        response = requests.post(cost_management_endpoint, json=query, headers=headers)
        if response.status_code != 200:
            print("Error:", response.status_code, response.text)


"""
for subscription in subscriptions:
    print("***" + subscription.subscription_id)
    resource_client = ResourceManagementClient(credential, subscription.subscription_id)
    resource_groups = list(resource_client.resource_groups.list())
    for rg in resource_groups:
        print(rg)

credential = DefaultAzureCredential()
subscription_client = SubscriptionClient(credential)
subscriptions = list(subscription_client.subscriptions.list())
#for subscription in subscriptions:
#    print(subscription)
# {'additional_properties': {}, 'id': '/subscriptions/64db6607-2ebe-48ad-be1b-d32e5fd1897c', 'subscription_id': '64db6607-2ebe-48ad-be1b-d32e5fd1897c', 'display_name': 'Azure subscription 1', 'tenant_id': '2d537187-9959-4d0a-a454-8fd82336fba2', 'state': 'Disabled', 'subscription_policies': <azure.mgmt.resource.subscriptions.v2021_01_01.models._models_py3.SubscriptionPolicies object at 0x7f2944386b50>, 'authorization_source': 'RoleBased', 'managed_by_tenants': [], 'tags': None}
# {'additional_properties': {}, 'id': '/subscriptions/a4a618df-464b-4b87-acbe-ccb077930906', 'subscription_id': 'a4a618df-464b-4b87-acbe-ccb077930906', 'display_name': 'Subscription-test', 'tenant_id': '2d537187-9959-4d0a-a454-8fd82336fba2', 'state': 'Enabled', 'subscription_policies': <azure.mgmt.resource.subscriptions.v2021_01_01.models._models_py3.SubscriptionPolicies object at 0x7f294435f850>, 'authorization_source': 'RoleBased', 'managed_by_tenants': [], 'tags': None}

***64db6607-2ebe-48ad-be1b-d32e5fd1897c
{'additional_properties': {}, 'id': '/subscriptions/64db6607-2ebe-48ad-be1b-d32e5fd1897c/resourceGroups/DefaultResourceGroup-EUS', 'name': 'DefaultResourceGroup-EUS', 'type': 'Microsoft.Resources/resourceGroups', 'properties': <azure.mgmt.resource.resources.v2022_09_01.models._models_py3.ResourceGroupProperties object at 0x7f722f63b290>, 'location': 'eastus', 'managed_by': None, 'tags': None}
***a4a618df-464b-4b87-acbe-ccb077930906
{'additional_properties': {}, 'id': '/subscriptions/a4a618df-464b-4b87-acbe-ccb077930906/resourceGroups/finops-rg', 'name': 'finops-rg', 'type': 'Microsoft.Resources/resourceGroups', 'properties': <azure.mgmt.resource.resources.v2022_09_01.models._models_py3.ResourceGroupProperties object at 0x7f722ea5e150>, 'location': 'canadaeast', 'managed_by': None, 'tags': {}}
"""
