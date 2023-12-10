import boto3

ce_client = boto3.client("ce")
start_date = "2023-11-01"
end_date = "2023-12-01"

dimensions = [
    {"Type": "DIMENSION", "Key": "SERVICE"},
    {"Type": "TAG", "Key": "user.projet"},
]
filter = {"Dimensions": {"Key": "LINKED_ACCOUNT", "Values": ["323625553814"]}}

response = ce_client.get_cost_and_usage(
    TimePeriod={
        "Start": start_date,
        "End": end_date,
    },
    Granularity="DAILY",
    Metrics=["BlendedCost"],
    GroupBy=dimensions,
    Filter=filter,
)

for result_by_time in response["ResultsByTime"]:
    #    print(result_by_time)
    for group in result_by_time["Groups"]:
        print(group)

# {'TimePeriod': {'Start': '2023-11-01', 'End': '2023-11-02'}, 'Total': {}, 'Groups': [{'Keys': ['Tax', 'user.projet$'], 'Metrics': {'BlendedCost': {'Amount': '2.13', 'Unit': 'USD'}}}], 'Estimated': False}
# {'Keys': ['Tax', 'user.projet$'], 'Metrics': {'BlendedCost': {'Amount': '2.13', 'Unit': 'USD'}}}
# {'TimePeriod': {'Start': '2023-11-02', 'End': '2023-11-03'}, 'Total': {'BlendedCost': {'Amount': '0', 'Unit': 'USD'}}, 'Groups': [], 'Estimated': False}
# {'TimePeriod': {'Start': '2023-11-03', 'End': '2023-11-04'}, 'Total': {'BlendedCost': {'Amount': '0', 'Unit': 'USD'}}, 'Groups': [], 'Estimated': False}
# {'TimePeriod': {'Start': '2023-11-04', 'End': '2023-11-05'}, 'Total': {'BlendedCost': {'Amount': '0', 'Unit': 'USD'}}, 'Groups': [], 'Estimated': False}
# {'TimePeriod': {'Start': '2023-11-05', 'End': '2023-11-06'}, 'Total': {'BlendedCost': {'Amount': '0', 'Unit': 'USD'}}, 'Groups': [], 'Estimated': False}
