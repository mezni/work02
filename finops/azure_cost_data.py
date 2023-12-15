import requests
from datetime import datetime


class AzureCostManagement:
    def __init__(self, subscription_id, client_id, client_secret, tenant_id):
        self.subscription_id = subscription_id
        self.client_id = client_id
        self.client_secret = client_secret
        self.tenant_id = tenant_id
        self.access_token = self.get_access_token()

    def get_access_token(self):
        url = f"https://login.microsoftonline.com/{self.tenant_id}/oauth2/token"
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        data = {
            "grant_type": "client_credentials",
            "client_id": self.client_id,
            "client_secret": self.client_secret,
            "resource": "https://management.azure.com/",
        }

        response = requests.post(url, headers=headers, data=data)
        response.raise_for_status()
        return response.json().get("access_token")

    def get_cost_data(self, start_date, end_date):
        url = f"https://management.azure.com/subscriptions/{self.subscription_id}/providers/Microsoft.CostManagement/query?"
        headers = {
            "Authorization": f"Bearer {self.access_token}",
            "Content-Type": "application/json",
        }
        body = {
            "type": "Usage",
            "timeframe": "Custom",
            "timePeriod": {"from": start_date, "to": end_date},
            "dataset": {
                "granularity": "Daily",
                "aggregation": {"totalCost": {"name": "PreTaxCost", "function": "Sum"}},
            },
        }

        response = requests.post(url, headers=headers, json=body)
        response.raise_for_status()
        return response.json()


# Example usage:
subscription_id = "your_subscription_id"
client_id = "your_client_id"
client_secret = "your_client_secret"
tenant_id = "your_tenant_id"

cost_management = AzureCostManagement(
    subscription_id, client_id, client_secret, tenant_id
)

# Replace start_date and end_date with the desired time range
start_date = datetime.utcnow().isoformat()
end_date = (datetime.utcnow() - timedelta(days=30)).isoformat()

cost_data = cost_management.get_cost_data(start_date, end_date)
print(cost_data)
