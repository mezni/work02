import json, os
from datetime import datetime, timedelta
import boto3
from botocore.exceptions import ClientError

sts = boto3.client("sts")
s3 = boto3.client("s3")
ce = boto3.client("ce")
iam = boto3.client("iam")


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
    local_path = "/tmp"
    state_file_name = prefix + ".state"
    state_prev_file_name = prefix + "_prev.state"
    granularity = "DAILY"
    metrics = ["UnblendedCost"]
    dimensions = [
        {"Type": "DIMENSION", "Key": "LINKED_ACCOUNT"},
        {"Type": "DIMENSION", "Key": "SERVICE"},
    ]

    try:
        account_id = sts.get_caller_identity()["Account"]
    except:
        status["code"] = 400
        status["message"] = "cannot get account_id"

    client_name = os.environ.get("client_name")
    if not client_name:
        status["code"] = 400
        status["message"] = "client_name is not defined"

    store_account_name = os.environ.get("store_account_name")
    if not store_account_name:
        status["code"] = 400
        status["message"] = "store_account_name is not defined"

    store_container_name = os.environ.get("store_container_name")
    if not store_container_name:
        status["code"] = 400
        status["message"] = "store_container_name is not defined"

    store_account_key = os.environ.get("store_account_key")
    if not store_account_key:
        status["code"] = 400
        status["message"] = "store_account_key is not defined"

    if status["code"] == 200:
        date_suffix = start_ts.strftime("%Y%m%d")
        file_name = (
            prefix
            + "_"
            + client_name
            + "_"
            + cloud_provider
            + "_"
            + account_id
            + "_"
            + date_suffix
            + "."
            + file_format
        )
        bucket_name = prefix + "-" + client_name + "-" + account_id
        local_file = local_path + "/" + file_name
        config = {
            "start_ts": start_ts,
            "client_name": client_name,
            "account_id": account_id,
            "store_account_name": store_account_name,
            "store_container_name": store_container_name,
            "store_account_key": store_account_key,
            "cloud_provider": cloud_provider,
            "file_name": file_name,
            "file_format": file_format,
            "local_file": local_file,
            "state_file_name": state_file_name,
            "state_prev_file_name": state_prev_file_name,
            "bucket_name": bucket_name,
            "query_duration": 7,
            "granularity": granularity,
            "metrics": metrics,
            "dimensions": dimensions,
            "start_date": "",
            "end_date": "",
        }

    return config, status


def read_file_from_bucket(bucket_name, file_name):
    step = "read_file_from_bucket"
    status = {"code": 200, "step": step, "message": "success"}
    try:
        response = s3.get_object(Bucket=bucket_name, Key=file_name)
        content = response["Body"].read().decode("utf-8")
    except ClientError as e:
        content = {}
        status["code"] = 400
        status["step"] = step
        status["message"] = e.response["Error"]["Message"]
    return content, status


def write_file_to_bucket(bucket_name, file_name, content):
    step = "write_file_to_bucket"
    status = {"code": 200, "step": step, "message": "success"}
    try:
        response = s3.put_object(Bucket=bucket_name, Key=file_name, Body=content)
    except ClientError as e:
        status["code"] = 400  # e.response['Error']['Code']
        status["message"] = e.response["Error"]["Message"]
    return status


def get_query_dates(config):
    step = "write_file_to_bucket"
    status = {"code": 200, "step": step, "message": "success"}
    query_dates = {"start_date": "", "end_date": ""}

    query_duration = int(config["query_duration"])
    bucket_name = config["bucket_name"]
    state_file_name = config["state_file_name"]

    state_file_data, ret = read_file_from_bucket(bucket_name, state_file_name)
    try:
        current_state = json.loads(state_file_data)
    except:
        current_state = {}

    state_prev_file_name = config["state_prev_file_name"]
    state_prev_file_data, ret = read_file_from_bucket(bucket_name, state_prev_file_name)
    try:
        prev_state = json.loads(state_prev_file_data)
    except:
        prev_state = {}

    try:
        if current_state["execution"]["status"] == "success":
            last_end_date = current_state["params"]["end_date"]
        else:
            last_end_date = prev_state["params"]["end_date"]
    except:
        last_end_date = None

    if last_end_date:
        if last_end_date >= config["start_ts"].strftime("%Y-%m-%d"):
            status["code"] = 400
            status["step"] = step
            status["message"] = "last query date is:" + last_end_date
        else:
            start_date = last_end_date
            end_date = config["start_ts"].strftime("%Y-%m-%d")
    else:
        start_date = (config["start_ts"] - timedelta(days=query_duration)).strftime(
            "%Y-%m-%d"
        )
        end_date = config["start_ts"].strftime("%Y-%m-%d")
    query_dates["start_date"] = start_date
    query_dates["end_date"] = end_date

    return query_dates, status


def lambda_handler(event, context):
    start_ts = datetime.now()
    state = init_state(start_ts)
    config, status = init_config(start_ts)
    if status["code"] != 200:
        state["execution"]["status"] = "failed"
        state["execution"]["step"] = status["step"]
        state["execution"]["message"] = status["message"]

    if state["execution"]["status"] == "success":
        query_dates, status = get_query_dates(config)
        state["execution"]["start_date"] = query_dates["start_date"]
        state["execution"]["end_date"] = query_dates["end_date"]

    state["execution"]["end_time"] = datetime.now().strftime("%d-%m-%Y %H:%M:%S")
    state_content = json.dumps(state, indent=2)
    ret = write_file_to_bucket(
        config["bucket_name"], config["file_name"], state_content
    )
    print(state)
    start_date = "2023-11-01"
    end_date = "2023-11-30"
    #    response = ce.get_cost_and_usage(
    #        TimePeriod={
    #            'Start': start_date,
    #            'End': end_date
    #        },
    #        Granularity='DAILY',
    #        Metrics=['BlendedCost']
    #    )
    #    my_org = boto3.client('organizations')
    #    accounts_paginator = my_org.get_paginator('list_accounts')
    #    accounts_pages = accounts_paginator.paginate()
    #    for account_page in accounts_pages:
    #        for account in account_page['Accounts']:
    #            print (account)
    #    response = iam.list_users()
    #
    #    for user in response['Users']:
    #        user_name = user['UserName']
    #        user_arn = user['Arn']
    #        user_create_date = user['CreateDate']
    #        print (user_name, user_arn,user_create_date )

    response = iam.get_user(UserName="root")
    print(response)
    return {"statusCode": 200, "body": json.dumps("Hello from Lambda!")}
