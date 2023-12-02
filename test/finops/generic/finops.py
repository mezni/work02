import json, uuid
from datetime import datetime, timedelta
from azure.storage.blob import BlobServiceClient
import pprint


def upload_to_azure_blob(account_name, account_key, container_name, file_name):
    blob_name = file_name.split("/")[-1]
    connection_string = f"DefaultEndpointsProtocol=https;AccountName={account_name};AccountKey={account_key};EndpointSuffix=core.windows.net"

    blob_service_client = BlobServiceClient.from_connection_string(connection_string)
    container_client = blob_service_client.get_container_client(container_name)
    with open(file_name, "rb") as data:
        blob_client = container_client.get_blob_client(blob_name)
        blob_client.upload_blob(data)


def init_state(params):
    prefix = "finops"
    start_ts = datetime.now()
    account_id = ""
    run_id = str(uuid.uuid4())
    bucket_store = "finops" + "-" + params["client_name"] + "-" + account_id
    data_file = (
        "finops"
        + "_"
        + params["client_name"]
        + "_"
        + params["cloud_provider"]
        + "_"
        + start_ts.strftime("%Y%m%d")
        + "."
        + params["data_format"]
    )
    state = {
        "execution": {
            "run_id": run_id,
            "start_time": start_ts.strftime("%d/%m/%Y %H:%M:%S"),
            "end_time": "",
            "status": "success",
            "message": "",
            "data_file": data_file,
            "data_format": params["data_format"],
        },
        "params": {
            "start_date": "",
            "end_date": "",
            "granularity": params["granularity"],
            "metrics": params["metrics"],
            "dimensions": [d["Key"] for d in params["dimensions"]],
            "client_name": params["client_name"],
            "cloud_provider": params["cloud_provider"],
            "bucket_store": bucket_store,
            "sink_account_name": params["sink_account_name"],
            "sink_container_name": params["sink_container_name"],
        },
    }
    return state


def read_file_from_bucket(bucket_name, file_name):
    return None


def get_query_dates(state):
    ret_code = 200
    ret_message = ""
    current_date = datetime.now()
    bucket_name = state["params"]["bucket_store"]
    old_state_file_name = "finops_old.state"
    current_state_file_name = "finops_current.state"
    old_state_content = read_file_from_bucket(bucket_name, old_state_file_name)
    try:
        old_state = json.loads(old_state_content)
    except:
        old_state = {}

    current_state_content = read_file_from_bucket(bucket_name, current_state_file_name)
    try:
        current_state = json.loads(current_state_content)
    except:
        current_state = {}

    try:
        if current_state["execution"]["status"] == "success":
            last_end_date = current_state["params"]["end_date"]
        else:
            last_end_date = old_state["params"]["end_date"]
    except:
        last_end_date = None

    if last_end_date:
        if last_end_date >= current_date.strftime("%Y-%m-%d"):
            ret_code = 400
            ret_message = "last query date is:" + last_end_date
        else:
            start_date = last_end_date
            end_date = current_date.strftime("%Y-%m-%d")
    else:
        start_date = (current_date - timedelta(days=7)).strftime("%Y-%m-%d")
        end_date = current_date.strftime("%Y-%m-%d")
    return start_date, end_date, ret_code, ret_message


# MAIN

params = {
    "data_format": "csv",
    "sink_account_name": "finopsstorageaccount2003",
    "sink_container_name": "finopscontainer",
    "sink_account_key": "MDPnx/y+PPsv63rnYL1kiepAtc/w196OIwu1dZHrCqBI1Kjz562Ja/iUDqdk9a2zExLWaKJGKLMn+AStEaNtfg==",
    "client_name": "client1",
    "cloud_provider": "aws",
    "granularity": "DAILY",
    "metrics": ["UnblendedCost"],
    "dimensions": [
        {"Type": "DIMENSION", "Key": "LINKED_ACCOUNT"},
        {"Type": "DIMENSION", "Key": "SERVICE"},
    ],
}

state = init_state(params)
start_date, end_date, ret_code, ret_message = get_query_dates(state)
state["params"]["start_date"] = start_date
state["params"]["end_date"] = end_date
if ret_code != 200:
    state["execution"]["status"] = "failed"
    state["execution"]["message"] = ret_message
else:
    pass

state["execution"]["end_time"] = datetime.now().strftime("%d/%m/%Y %H:%M:%S")
upload_to_azure_blob(
    state["params"]["sink_account_name"],
    params["sink_account_key"],
    state["params"]["sink_container_name"],
    "/tmp/test.txt",
)
# upload_to_azure_blob(account_name, account_key, container_name, file_name)
pprint.pprint(state)
