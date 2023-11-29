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


def write_s3_jsonfile(bucket_name, file_name, checkpoint):
    """write s3 file"""
    json_string = json.dumps(checkpoint, indent=2)
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


def init_config(current_ts, days_back):
    """init config"""
    granurality = "DAILY"
    metrics = ["UnblendedCost"]
    dimensions = [
        {"Type": "DIMENSION", "Key": "LINKED_ACCOUNT"},
        {"Type": "DIMENSION", "Key": "SERVICE"},
    ]
    config = {}
    account_id = sts.get_caller_identity()["Account"]
    start_date = (current_ts - timedelta(days=days_back)).strftime("%Y-%m-%d")
    end_date = current_ts.strftime("%Y-%m-%d")
    suffix_date = current_ts.strftime("%Y%m%d")
    output_file = "finsops_aws_" + str(account_id) + "_" + suffix_date + ".csv"
    bucket_name = "finsops-" + str(account_id)
    config["start_date"] = start_date
    config["end_date"] = end_date
    config["output_file"] = output_file
    config["bucket_name"] = bucket_name
    config["granurality"] = granurality
    config["metrics"] = metrics
    config["dimensions"] = dimensions
    return config


def init_checkpoint(config, current_ts):
    checkpoint = {}
    checkpoint["execution"] = {}
    checkpoint["execution"]["run_start"] = current_ts.strftime("%d-%m-%Y %H:%M:%S")
    checkpoint["execution"]["run_end"] = datetime.now().strftime("%d-%m-%Y %H:%M:%S")
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


current_ts = datetime.now()
days_back = 1
config = init_config(current_ts, days_back)
checkpoint = init_checkpoint(config, current_ts)
print(checkpoint)
