import json
from datetime import datetime, timedelta


def read_app_config():
    with open("finops.conf", "r") as file:
        config = json.load(file)
    return config


def list_clients():
    clients = set()
    for c in app_config["clients"]:
        clients.add(c["client_name"])
    return list(clients)


columns = [
    "Period",
    "Client",
    "Cloud",
    "Resource",
    "ResourceId",
    "ResourceType",
    "ResourceGroupName",
    "ResourceGroupId",
    "ResourceLocation",
    "SubscriptionName",
    "SubscriptionId",
    "Tags",
    "ServiceName",
    "ServiceTier",
    "Product",
    "Meter",
    "Cost",
    "CostUSD",
    "Currency",
]


# MAIN
start_ts = datetime.now()

app_config = read_app_config()

real_flag = app_config["app"]["real"]
clients = list_clients()

for client in clients:
    for c in app_config["clients"]:
        if c["client_name"] == client:
            if c["cloud_name"] == "azure":
                if not real_flag:
                    print("fake azure")
                else:
                    print("real azure")
            if c["cloud_name"] == "aws":
                if not real_flag:
                    print("fake aws")
                else:
                    print("real aws")
