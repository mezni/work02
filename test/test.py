from azure.identity import DefaultAzureCredential
from azure.mgmt.costmanagement import CostManagementClient


def get_azure_cost_data(subscription_id, start_date, end_date):
    # Use Azure Default Credential for authentication
    credential = DefaultAzureCredential()

    # Create a CostManagementClient
    cost_client = CostManagementClient(credential, subscription_id)

    # Set the time period for the query
    timeframe = {"start": start_date, "end": end_date}

    # Set the granularity of the query
    granularity = "Daily"

    # Specify the metrics to retrieve
    metrics = ["AmortizedCost"]

    # Specify the dimension to group by (optional)
    dimensions = ["ResourceId"]

    # Build the query
    query_body = {
        "type": "Usage",
        "timeframe": timeframe,
        "dataset": {
            "granularity": granularity,
            "aggregation": {"totalCost": {"name": "PreTaxCost", "function": "Sum"}},
        },
        "metrics": metrics,
        "dimensions": dimensions,
    }

    # Retrieve the data
    result = cost_client.query.usage(subscription_id, parameters=query_body)

    # Print the results
    for row in result.rows:
        print(
            f"Resource ID: {row.dimension_values[0]}, Cost: {row.values[0]} {result.columns[0].unit}"
        )


if __name__ == "__main__":
    # Set your Azure subscription ID
    subscription_id = "a4a618df-464b-4b87-acbe-ccb077930906"

    # Set the start and end dates for the query
    start_date = "2023-12-01"
    end_date = "2023-12-31"

    # Call the function to get cost data
    get_azure_cost_data(subscription_id, start_date, end_date)
