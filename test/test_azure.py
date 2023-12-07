import requests
from azure.identity import DefaultAzureCredential
from azure.mgmt.resource import SubscriptionClient, ResourceManagementClient
from azure.mgmt.consumption import ConsumptionManagementClient
from datetime import datetime, timedelta


def get_subscriptions(credential):
    subscription_client = SubscriptionClient(credential)
    subscriptions = list(subscription_client.subscriptions.list())
    return subscriptions


def get_resource_groups(credential, subscription_id):
    resource_client = ResourceManagementClient(credential, subscription_id)
    resource_groups = list(resource_client.resource_groups.list())
    return resource_groups


def get_azure_costs(credential, subscription_id, scope, start_date, end_date):
    consumption_client = ConsumptionManagementClient(credential, subscription_id)

    filter_date_range = "usageEnd eq {} and usageStart eq {}".format(
        end_date, start_date
    )
    usage_details = consumption_client.usage_details.list(
        scope=scope, filter=filter_date_range
    )
    for usage_detail in usage_details:
        print(f"Resource ID: {usage_detail.resource_id}")
        print(f"Usage Start: {usage_detail.usage_start}")
        print(f"Usage End: {usage_detail.usage_end}")
        print(f"Meter ID: {usage_detail.meter_id}")
        print(f"Quantity: {usage_detail.quantity}")
        print(f"Meter Details: {usage_detail.meter_details}")
        print("------------------------------")


credential = DefaultAzureCredential()
subscription_client = SubscriptionClient(credential)
subscriptions = list(subscription_client.subscriptions.list())
for subscription in subscriptions:
    #    print(subscription)
    #    print(subscription.display_name)
    print(subscription.subscription_id)
    print(subscription.tenant_id)
    #    print(subscription.state)
    resource_groups = get_resource_groups(credential, subscription.subscription_id)
    for resource_group in resource_groups:
        print(resource_group)
        print(resource_group.name)
        print(resource_group.location)
        print(resource_group.tags)
        end_date = datetime.utcnow()
        start_date = end_date - timedelta(days=30)
        scope = "/subscriptions/{subscription.subscription_id}/resourceGroups/{resource_group.name}"
#        get_azure_costs(
#            credential, subscription.subscription_id, scope, start_date, end_date
#        )
