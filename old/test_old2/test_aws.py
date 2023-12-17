## creer un env virtuel
## installer boto3 - pip install boto3
## changer les date de debut et de fin
## export region_name="ca-central-1"
## export aws_access_key_id=""
## export aws_secret_access_key=""

import os
import json
import boto3

region_name = os.getenv("region_name")
aws_access_key_id = os.getenv("aws_access_key_id")
aws_secret_access_key = os.getenv("aws_secret_access_key")


def format(cost_data):
    columns = ["periode", "compte", "service", "cout", "devise", "estimation"]
    print(",".join(columns))
    for result_by_time in cost_data["ResultsByTime"]:
        for group in result_by_time["Groups"]:
            period = result_by_time["TimePeriod"]["Start"]
            account = group["Keys"][0]
            service = group["Keys"][1]
            amount = group["Metrics"]["UnblendedCost"]["Amount"]
            unit = group["Metrics"]["UnblendedCost"]["Unit"]
            estimated = result_by_time["Estimated"]
            line = (
                period
                + ","
                + account
                + ","
                + service
                + ","
                + amount
                + ","
                + unit
                + ","
                + str(estimated)
            )
            print(line)


ce_client = boto3.client(
    "ce",
    region_name=region_name,
    aws_access_key_id=aws_access_key_id,
    aws_secret_access_key=aws_secret_access_key,
)

time_period = {"Start": "2023-12-06", "End": "2023-12-07"}

account_id = "323625553814"

granularity = "DAILY"
metrics = ["UnblendedCost"]
group_by = [
    {"Type": "DIMENSION", "Key": "SERVICE"},
    {"Type": "TAG", "Key": "user:projet"},
]
filter = {"Dimensions": {"Key": "LINKED_ACCOUNT", "Values": [account_id]}}

cost_data = ce_client.get_cost_and_usage(
    TimePeriod=time_period,
    Granularity=granularity,
    Metrics=metrics,
    GroupBy=group_by,
    Filter=filter,
)

print(cost_data)
# format(cost_data)
