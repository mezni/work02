import os
import boto3
from datetime import datetime, timedelta


class CostAws:
    def __init__(self, context) -> None:
        self.start_time = datetime.now()
        self.end_time = None
        self.status = None
        self.message = None
        self.credentials = context.get("credentials")
        self.params = context.get("params")
        self.ce_client = self.create_ce_client()

    def create_ce_client(self):
        try:
            return boto3.client(
                "ce",
                aws_access_key_id=self.credentials.get("access_key_id"),
                aws_secret_access_key=self.credentials.get("secret_access_key"),
                region_name=self.credentials.get("region"),
            )
        except Exception as e:
            self.status = "failed"
            self.message = e
            return None

    def get_cost(self):
        results = []

        start_date = self.params.get("start_date")
        end_date = self.params.get("end_date")
        granularity = self.params.get("granularity")
        dimensions = [
            {"Type": "DIMENSION", "Key": d} for d in self.params.get("dimensions")
        ]
        metrics = self.params.get("metrics")
        filters = self.params.get("filters")

        token = None
        while True:
            if token:
                kwargs = {"NextPageToken": token}
            else:
                kwargs = {}

            try:
                data = self.ce_client.get_cost_and_usage(
                    TimePeriod={"Start": start_date, "End": end_date},
                    Granularity=granularity,
                    Metrics=metrics,
                    GroupBy=dimensions,
                    **kwargs
                )

                results += data["ResultsByTime"]
                token = data.get("NextPageToken")
            except Exception as e:
                self.status = "failed"
                self.message = e

            if not token:
                break

        return results


context = {
    "credentials": {
        "access_key_id": os.getenv("access_key_id"),
        "secret_access_key": os.getenv("secret_access_key"),
        "region": os.getenv("region"),
    },
    "params": {
        "start_date": "2023-12-16",
        "end_date": "2023-12-17",
        "client_name": "test",
        "granularity": "DAILY",
        "dimensions": ["LINKED_ACCOUNT", "SERVICE"],
        "metrics": ["BlendedCost"],
        "filters": "",
    },
}

x = CostAws(context)
y = x.get_cost()
print(y)
print(x.status)
print(x.message)
