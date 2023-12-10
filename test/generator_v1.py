from datetime import datetime, timedelta
import random

clients = [
    {
        "client_name": "Quantum Innovations",
        "account_id": "323625553814",
        "account_seed": "1",
    },
    {
        "client_name": "Quantum Innovations",
        "account_id": "323621853859",
        "account_seed": "12",
    },
    {
        "client_name": "Quantum Innovations",
        "account_id": "323600053376",
        "account_seed": "67",
    },
    {
        "client_name": "Quantum Innovations",
        "account_id": "323652553090",
        "account_seed": "89",
    },
    {
        "client_name": "Novus Dynamics",
        "account_id": "332654133971",
        "account_seed": "12",
    },
    {
        "client_name": "Novus Dynamics",
        "account_id": "344655443862",
        "account_seed": "30",
    },
    {
        "client_name": "Novus Dynamics",
        "account_id": "376657773753",
        "account_seed": "48",
    },
    {
        "client_name": "Horizon Innovations",
        "account_id": "311653278753",
        "account_seed": "57",
    },
    {
        "client_name": "Horizon Innovations",
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
    "end_date": "2023-12-12",
}


def aws_cost_data():
    cloud_name = "aws"
    columns = ["periode", "client", "cloud", "compte", "service", "cout", "devise"]
    header = ",".join(columns)
    file_name = (
        +"_"
        + datetime.strptime(config["end_date"], "%Y-%m-%d").strftime("%Y-%m-%d")
        + ".csv"
    )
    with open(file_name, "a") as file:
        # Append content to the file
        file.write(header + "\n")
        for client in clients:
            seed = int(client["account_seed"])
            start_date_ts = datetime.strptime(config["start_date"], "%Y-%m-%d")
            end_date_ts = datetime.strptime(config["end_date"], "%Y-%m-%d")

            while start_date_ts.strftime("%Y-%m-%d") < config["end_date"]:
                dayofyear = int(start_date_ts.timetuple().tm_yday)
                day_name = start_date_ts.strftime("%A")
                for service in services:
                    cost = (
                        float(service["price"])
                        * seed
                        * random.randint(0, 10)
                        * dayofyear
                    )
                    if day_name == "Saturday":
                        cost = cost * 0.6
                    if day_name == "Sunday":
                        cost = cost * 0.4

                    line = (
                        start_date_ts.strftime("%Y-%m-%d")
                        + ","
                        + client["client_name"]
                        + ","
                        + cloud_name
                        + ","
                        + client["account_id"]
                        + ","
                        + service["service"]
                        + ","
                        + str(cost)
                        + ","
                        + "USD"
                    )
                    file.write(line + "\n")

                start_date_ts = start_date_ts + timedelta(days=1)


aws_cost_data()
