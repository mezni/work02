import json
from datetime import datetime, timedelta


def read_app_config(config_file):
    with open(config_file, "r") as file:
        config = json.load(file)
    return config


def list_clients(app_config):
    clients = set()
    for c in app_config["clients"]:
        clients.add(c["client_name"])
    return list(clients)


def generate_data(client, real_flag):
    result = []
    cloud_name = client["cloud_name"]
    client_name = client["client_name"]
    client_code = client["client_code"]
    print(cloud_name)
    return result


def main():
    start_ts = datetime.now()
    real_flag = app_config["app"]["real"]
    config_file = "finops.conf"
    app_config = read_app_config(config_file)
    clients = list_clients(app_config)
    for c in clients:
        for client in app_config["clients"]:
            if client["client_name"] == c:
                result = generate_data(client, real_flag)


if __name__ == "__main__":
    main()
