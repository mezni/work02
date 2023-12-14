import yaml, random
from datetime import datetime, timedelta
from azure.storage.blob import BlobClient

__author__ = "Mohamed Ali MEZNI"
__version__ = "2023-12-12"


def read_config_file(file_name):
    step = "read_config_file"
    status = {"code": 200, "step": step, "message": "success"}
    try:
        with open(file_name, "r") as file:
            data = yaml.safe_load(file)
    except:
        status = {"code": 400, "step": step, "message": "config file does not exist"}
        data = {}
    return data, status


def get_clients(config):
    clients = set()
    for client in config["clients"]:
        clients.add(client["client_code"])
    return list(clients)


def init_state():
    state = {}
    state["execution"] = {}
    state["execution"]["start_time"] = datetime.now().strftime("%d-%m-%Y %H:%M:%S")
    state["execution"]["end_time"] = ""
    state["execution"]["status"] = "success"
    state["execution"]["message"] = ""
    state["params"] = {}
    state["params"]["start_date"] = ""
    state["params"]["end_date"] = ""
    state["params"]["granularity"] = ""
    state["params"]["metrics"] = ""
    state["params"]["dimensions"] = ""
    state["params"]["start_date"] = ""
    state["params"]["end_date"] = ""
    return state


def cost_data_aws_fake(config):
    data = []
    services = [
        {"service": "Amazon EC2 (Elastic Compute Cloud)", "price": "0.00057"},
        {"service": "Amazon S3 (Simple Storage Service)", "price": "0.00023"},
        {"service": "Amazon RDS (Relational Database Service)", "price": "0.00012"},
        {"service": "Amazon Lambda", "price": "0.000033"},
        {"service": "Amazon DynamoDB", "price": "0.000078"},
        {"service": "Amazon ElastiCache", "price": "0.000012"},
        {"service": "Amazon Route 53", "price": "0.000001"},
    ]

    try:
        seed = int(config["account_seed"])
    except:
        seed = 1

    start_date_ts = datetime.strptime("2023-06-01", "%Y-%m-%d")
    end_date = datetime.now().strftime("%Y-%m-%d")

    while start_date_ts.strftime("%Y-%m-%d") < end_date:
        dayofyear = int(start_date_ts.timetuple().tm_yday)
        day_name = start_date_ts.strftime("%A")
        for service in services:
            cost = float(service["price"]) * seed * random.randint(1, 5) * dayofyear

            if day_name == "Saturday":
                cost = cost * 0.6
            if day_name == "Sunday":
                cost = cost * 0.4
            cost_entry = {
                "period": start_date_ts.strftime("%Y-%m-%d"),
                "client": config["client"]["client_name"],
                "cloud": config["client"]["cloud_name"],
                "account_id": config["client"]["account_id"],
                "service": service["service"],
                "cost": f"{cost:.6f}",
                "unit": "USD",
            }
            data.append(cost_entry)
        start_date_ts = start_date_ts + timedelta(days=1)
    return data


def cost_data_aws(config):
    data = []
    print("cost_data_aws")
    return data


def generate(cost_data, file_name):
    file_path = "temp.tmp"

    with open(file_path, "w") as file:
        columns = list(cost_data[0].keys())
        cost_header = ",".join(columns)
        file.write(cost_header + "\n")
        for item in cost_data:
            line = []
            for col in columns:
                line.append(item[col])
            cost_line = ",".join(line)
            file.write(cost_line + "\n")

    blob_client = BlobClient.from_connection_string(
        conn_str="DefaultEndpointsProtocol=https;AccountName=finopsblobstorageaccount;AccountKey=XXXXXXXXX;EndpointSuffix=core.windows.net",
        container_name="bronze",
        blob_name=file_name,
    )

    with open("temp.tmp", "rb") as data:
        blob_client.upload_blob(data)


def process_account(config):
    state = init_state()
    client = config["client"]
    if client.get("fake_data") is not None:
        fake_data = client["fake_data"]
    else:
        fake_data = False

    if client.get("account_seed") is not None:
        account_seed = client["account_seed"]
    else:
        account_seed = 1

    if client.get("cloud_name") == "aws":
        print(state)
        file_name = (
            "finops_"
            + client["client_code"]
            + "_"
            + client["cloud_name"]
            + "_"
            + client["account_id"]
            + "_"
            + datetime.now().strftime("%Y%m%d")
            + ".csv"
        )
        if fake_data:
            cost_data = cost_data_aws_fake(config)
        else:
            cost_data = cost_data_aws(config)
        generate(cost_data, file_name)

    state["execution"]["end_time"] = datetime.now().strftime("%d-%m-%Y %H:%M:%S")


def main():
    start_ts = datetime.now()
    app_config, status = read_config_file(file_name)
    if status["code"] == 200:
        try:
            app_name = app_config["app"]["name"]
            #        clients = get_clients(app_config)
            for client in app_config["clients"]:
                account_config = {
                    "start_ts": start_ts,
                    "app_name": app_name,
                    "client": client,
                }
                process_account(account_config)
        except:
            pass


file_name = "config.yaml"

if __name__ == "__main__":
    main()
