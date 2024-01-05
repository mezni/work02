import boto3
from datetime import datetime, timedelta


class CostAws:
    def __init__(self, context) -> None:
        self.start_time = datetime.now()
        self.end_time = None
        self.status = None
        self.message = None
        self.aws_access_key_id = context["credential"].get("access_key_id")
        self.aws_secret_access_key = context["credential"].get("secret_access_key")
        self.aws_region = context["credential"].get("region")
        self.params = context.get("params")

    #        self.ce_client = self.create_ce_client()

    def create_ce_client(self):
        try:
            return boto3.client(
                "ce",
                aws_access_key_id=self.aws_access_key_id,
                aws_secret_access_key=self.aws_secret_access_key,
                region_name=self.aws_region,
            )
        except Exception as e:
            self.status = "failed"
            self.message = e
            return None

    def get_cost(self):
        results = []

        start_date = ""
        end_date = ""
        granularity = ""
        dimensions = ""
        metrics = ""
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
    "credential": {
        "access_key_id": "XXXX",
    }
}

x = CostAws(context)
print(x.params)
