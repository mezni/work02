import json
from datetime import datetime, timedelta
import boto3

sts = boto3.client("sts")
s3 = boto3.client("s3")
ce = boto3.client("ce")


def check_bucket_existance(bucket_name):
    """check bucket existance"""
    status = True
    try:
        s3.head_bucket(Bucket=bucket_name)

    except:
        status = False

    return status


def read_s3_jsonfile(bucket_name, file_name):
    """read s3 file"""
    try:
        response = s3.get_object(Bucket=bucket_name, Key=file_name)
        json_data = response["Body"].read().decode("utf-8")
        json_dict = json.loads(json_data)
    except:
        json_dict = {}

    return json_dict


def write_s3_jsonfile(bucket_name, file_name, content):
    """write s3 file"""
    json_string = json.dumps(content, indent=2)
    result = s3.put_object(Bucket=bucket_name, Key=file_name, Body=json_string)
    if result["ResponseMetadata"]["HTTPStatusCode"] == 200:
        status = True
    else:
        status = False
    return status


def write_s3_file(bucket_name, file_name, source_file):
    """write s3 file"""
    with open("/tmp/" + source_file, "rb") as file_content:
        result = s3.put_object(Body=file_content, Bucket=bucket_name, Key=file_name)

    if result["ResponseMetadata"]["HTTPStatusCode"] == 200:
        status = True
    else:
        status = False
    return status


def get_cost_data(start_date, end_date, granularity, metrics, dimensions):
    """generate cost explorer data"""
    results = []

    token = None
    while True:
        if token:
            kwargs = {"NextPageToken": token}
        else:
            kwargs = {}

        data = ce.get_cost_and_usage(
            TimePeriod={"Start": start_date, "End": end_date},
            Granularity=granularity,
            Metrics=metrics,
            GroupBy=dimensions,
            **kwargs
        )

        results += data["ResultsByTime"]
        token = data.get("NextPageToken")
        if not token:
            break

    return results


def init_config(current_ts, granurality, metrics, dimensions):
    """init config"""
    config = {}
    account_id = sts.get_caller_identity()["Account"]
    #    start_date = (current_ts - timedelta(days=days_back)).strftime("%Y-%m-%d")
    #    end_date = current_ts.strftime("%Y-%m-%d")
    start_date = ""
    end_date = ""
    suffix_date = current_ts.strftime("%Y%m%d")
    file_name = "finsops_aws_" + str(account_id) + "_" + suffix_date + ".csv"
    bucket_name = "finsops-" + str(account_id)
    config["start_date"] = start_date
    config["end_date"] = end_date
    config["file_name"] = file_name
    config["bucket_name"] = bucket_name
    config["granurality"] = granurality
    config["metrics"] = metrics
    config["dimensions"] = dimensions
    return config


def init_checkpoint(config, current_ts):
    checkpoint = {}
    checkpoint["execution"] = {}
    checkpoint["execution"]["run_start"] = current_ts.strftime("%d-%m-%Y %H:%M:%S")
    checkpoint["execution"]["status"] = "success"
    checkpoint["execution"]["message"] = ""
    checkpoint["ce_query_params"] = {}
    checkpoint["ce_query_params"]["start_date"] = config["start_date"]
    checkpoint["ce_query_params"]["end_date"] = config["end_date"]
    checkpoint["ce_query_params"]["granurality"] = config["granurality"]
    checkpoint["ce_query_params"]["metrics"] = config["metrics"]
    checkpoint["ce_query_params"]["dimensions"] = [
        d["Key"] for d in config["dimensions"]
    ]

    return checkpoint


def process_cost_data(cost_data, bucket_name, file_name):
    temp_path = "/tmp"
    temp_file = temp_path + "/" + file_name
    columns = ["period", "account", "service", "amount", "unit", "estimated"]
    header = ",".join(columns)
    with open(temp_file, "a") as file:
        file.write(header + "\n")

        for result_by_time in cost_data:
            for group in result_by_time["Groups"]:
                priod = result_by_time["TimePeriod"]["Start"]
                account = group["Keys"][0]
                service = group["Keys"][1]
                amount = group["Metrics"]["UnblendedCost"]["Amount"]
                unit = group["Metrics"]["UnblendedCost"]["Unit"]
                estimated = result_by_time["Estimated"]
                line = (
                    priod
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
                file.write(line + "\n")
    status = write_s3_file(bucket_name, file_name, temp_file)
    return status


# Main
checkpoint_file_name = "checkpoint.json"
checkpoint_previous_file_name = "checkpoint_previous.json"
execution_status = "success"
execution_message = ""

granurality = "DAILY"
metrics = ["UnblendedCost"]
dimensions = [
    {"Type": "DIMENSION", "Key": "LINKED_ACCOUNT"},
    {"Type": "DIMENSION", "Key": "SERVICE"},
]

current_ts = datetime.now()

config = init_config(current_ts)
bucket_name = config["bucket_name"]
file_name = config["file_name"]

checkpoint_current = read_s3_jsonfile(bucket_name, checkpoint_file_name)
checkpoint_previous = read_s3_jsonfile(bucket_name, checkpoint_previous_file_name)

if checkpoint_current["execution"]["status"] == "success":
    status = write_s3_jsonfile(
        bucket_name, checkpoint_previous_file_name, checkpoint_current
    )
checkpoint = init_config(current_ts, granurality, metrics, dimensions)

if not check_bucket_existance(bucket_name):
    execution_status = "failed"
    execution_message = "cannot access bucket " + bucket_name
    checkpoint["execution"]["status"] = execution_status
    checkpoint["execution"]["message"] = execution_message

if checkpoint["execution"]["status"] == "success":
    try:
        last_start_date = checkpoint_current["ce_query_params"]["start_date"]
        last_end_date = checkpoint_current["ce_query_params"]["end_date"]
    except:
        last_start_date = ""
        last_end_date = ""
    current_date = current_ts.strftime("%Y-%m-%d")
    if last_end_date == current_date:
        execution_status = "failed"
        execution_message = "last end date=" + last_end_date
    else:
        if last_end_date == "":
            start_date = (current_ts - timedelta(days=365)).strftime("%Y-%m-%d")
        else:
            start_date = last_end_date
        end_date = current_date

        checkpoint["ce_query_params"]["start_date"] = start_date
        checkpoint["ce_query_params"]["end_date"] = end_date
        cost_data = get_cost_data(
            start_date, end_date, granurality, metrics, dimensions
        )
        if not process_cost_data(cost_data, bucket_name, file_name):
            execution_status = "failed"
            execution_message = (
                "cannot write file " + file_name + " to s3 bucket " + bucket_name
            )
            checkpoint["execution"]["status"] = execution_status
            checkpoint["execution"]["message"] = execution_message

checkpoint["execution"]["run_end"] = datetime.now().strftime("%d-%m-%Y %H:%M:%S")
write_s3_jsonfile(bucket_name, file_name, checkpoint)


# Clean Up > 45days
# Rename file // add client finops_quebec_emploi_aws_20231128.csv
