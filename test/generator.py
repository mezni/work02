from datetime import datetime, timedelta
import random


def init_state(start_ts):
    state = {}
    state["execution"] = {}
    state["execution"]["start_time"] = start_ts.strftime("%d-%m-%Y %H:%M:%S")
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


def init_config(start_ts):
    step = "init_config"
    status = {"code": 200, "step": step, "message": "success"}

    config = {}
    prefix = "finops"
    cloud_provider = "aws"
    file_format = "csv"

    return config, status


def aws_cost_data(client):
    result = []

    seed = int(client["account_seed"])
    start_date_ts = datetime.strptime(config["start_date"], "%Y-%m-%d")
    while start_date_ts.strftime("%Y-%m-%d") < config["end_date"]:
        dayofyear = int(start_date_ts.timetuple().tm_yday)
        day_name = start_date_ts.strftime("%A")
        for service in services:
            cost = float(service["price"]) * seed * random.randint(1, 5) * dayofyear
            if day_name == "Saturday":
                cost = cost * 0.6
            if day_name == "Sunday":
                cost = cost * 0.4

            line = (
                start_date_ts.strftime("%Y-%m-%d")
                + ","
                + client["client_name"]
                + ","
                + client["cloud_name"]
                + ","
                + client["account_id"]
                + ","
                + service["service"]
                + ","
                + f"{cost:.6f}"
                + ","
                + "USD"
            )
            result.append(line)

        start_date_ts = start_date_ts + timedelta(days=1)

    return result


clients = [
    {
        "client_name": "Quantum Innovations",
        "client_code": "quantuminnovations",
        "cloud_name": "aws",
        "account_id": "323625553814",
        "account_seed": "1",
    },
    {
        "client_name": "Quantum Innovations",
        "client_code": "quantuminnovations",
        "cloud_name": "aws",
        "account_id": "323621853859",
        "account_seed": "12",
    },
    {
        "client_name": "Quantum Innovations",
        "client_code": "quantuminnovations",
        "cloud_name": "aws",
        "account_id": "323600053376",
        "account_seed": "67",
    },
    {
        "client_name": "Quantum Innovations",
        "client_code": "quantuminnovations",
        "cloud_name": "aws",
        "account_id": "323652553090",
        "account_seed": "89",
    },
    {
        "client_name": "Novus Dynamics",
        "client_code": "novusdynamics",
        "cloud_name": "aws",
        "account_id": "332654133971",
        "account_seed": "12",
    },
    {
        "client_name": "Novus Dynamics",
        "client_code": "novusdynamics",
        "cloud_name": "aws",
        "account_id": "344655443862",
        "account_seed": "30",
    },
    {
        "client_name": "Novus Dynamics",
        "client_code": "novusdynamics",
        "cloud_name": "aws",
        "account_id": "376657773753",
        "account_seed": "48",
    },
    {
        "client_name": "Horizon Innovations",
        "client_code": "horizoninnovations",
        "cloud_name": "aws",
        "account_id": "311653278753",
        "account_seed": "57",
    },
    {
        "client_name": "Horizon Innovations",
        "client_code": "horizoninnovations",
        "cloud_name": "aws",
        "account_id": "322653477654",
        "account_seed": "78",
    },
]

services = [
    {"service": "Amazon EC2 (Elastic Compute Cloud)", "price": "0.00057"},
    {"service": "Amazon S3 (Simple Storage Service)", "price": "0.00023"},
    {"service": "Amazon RDS (Relational Database Service)", "price": "0.00012"},
    {"service": "Amazon Lambda", "price": "0.000033"},
    {"service": "Amazon DynamoDB", "price": "0.000078"},
    {"service": "Amazon ElastiCache", "price": "0.000012"},
    {"service": "Amazon Route 53", "price": "0.000001"},
]


config = {
    "start_date": "2023-06-01",
    "end_date": "2023-12-11",
}


def main():
    start_ts = datetime.now()
    state = init_state(start_ts)
    client_codes_set = set()
    for client in clients:
        client_codes_set.add(client["client_code"])
    client_codes = list(client_codes_set)
    for client_code in client_codes:
        cost_data = []
        for client in clients:
            cloud_name = "aws"
            if (
                client_code == client["client_code"]
                and client["cloud_name"] == cloud_name
            ):
                result = aws_cost_data(client)
                cost_data = cost_data + result
        if cost_data:
            columns = [
                "periode",
                "client",
                "cloud",
                "compte",
                "service",
                "cout",
                "devise",
            ]
            header = ",".join(columns)

            file_name = (
                "finsops"
                + "_"
                + client_code
                + "_"
                + cloud_name
                + "_"
                + datetime.strptime(config["end_date"], "%Y-%m-%d").strftime("%Y%m%d")
                + ".csv"
            )
            with open(file_name, "a") as file:
                file.write(header + "\n")
                for line in cost_data:
                    file.write(line + "\n")


main()
