import boto3
from datetime import datetime, timedelta


class AWSCostExplorer:
    def __init__(
        self, aws_access_key_id, aws_secret_access_key, aws_region="us-east-1"
    ):
        self.aws_access_key_id = aws_access_key_id
        self.aws_secret_access_key = aws_secret_access_key
        self.aws_region = aws_region
        self.cost_explorer_client = self.create_cost_explorer_client()

    def create_cost_explorer_client(self):
        return boto3.client(
            "ce",
            aws_access_key_id=self.aws_access_key_id,
            aws_secret_access_key=self.aws_secret_access_key,
            region_name=self.aws_region,
        )

    def get_cost_data(self, start_date, end_date):
        response = self.cost_explorer_client.get_cost_and_usage(
            TimePeriod={"Start": start_date, "End": end_date},
            Granularity="DAILY",  # You can choose 'HOURLY', 'DAILY', or 'MONTHLY'
            Metrics=[
                "BlendedCost"
            ],  # You can customize the metrics based on your requirements
        )

        # Extract relevant information from the response
        results = response["ResultsByTime"]

        for result in results:
            print(f"Start Time: {result['TimePeriod']['Start']}")
            print(f"End Time: {result['TimePeriod']['End']}")
            print(
                f"Blended Cost: {result['Total']['BlendedCost']['Amount']} {result['Total']['BlendedCost']['Unit']}"
            )
            print("--------")


# Example usage:
aws_access_key_id = "your_aws_access_key_id"
aws_secret_access_key = "your_aws_secret_access_key"
aws_region = "your_aws_region"

cost_explorer = AWSCostExplorer(aws_access_key_id, aws_secret_access_key, aws_region)

# Replace start_date and end_date with the desired time range
start_date = (datetime.utcnow() - timedelta(days=30)).strftime("%Y-%m-%d")
end_date = datetime.utcnow().strftime("%Y-%m-%d")

cost_explorer.get_cost_data(start_date, end_date)
