from azure.identity import DefaultAzureCredential
from azure.mgmt.consumption import ConsumptionManagementClient
from datetime import datetime, timedelta


class AzureCostManagement:
    def __init__(self, subscription_id):
        self.subscription_id = subscription_id
        self.credentials = DefaultAzureCredential()
        self.consumption_client = self.create_consumption_client()

    def create_consumption_client(self):
        return ConsumptionManagementClient(self.credentials, self.subscription_id)

    def get_cost_data(self, start_date, end_date):
        result = self.consumption_client.usage_details.list(
            filter=f"usageEnd ge {start_date} and usageEnd le {end_date}"
        )

        return result


# Example usage:
subscription_id = "your_subscription_id"

cost_management = AzureCostManagement(subscription_id)

# Replace start_date and end_date with the desired time range
start_date = (datetime.utcnow() - timedelta(days=30)).strftime("%Y-%m-%dT%H:%M:%SZ")
end_date = datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")

try:
    cost_data = cost_management.get_cost_data(start_date, end_date)
    for item in cost_data:
        print(item)
except Exception as e:
    print(f"An error occurred: {e}")
