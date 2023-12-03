from datetime import datetime, timedelta


def init_config(params):
    start_ts = datetime.now()
    prefix = "finops"
    data_file_format = "csv"
    cloud_provider = "aws"
    granularity = "DAILY"
    metrics = ["UnblendedCost"]
    dimensions = [
        {"Type": "DIMENSION", "Key": "LINKED_ACCOUNT"},
        {"Type": "DIMENSION", "Key": "SERVICE"},
    ]
    current_state_file_name = "finops.state"
    previous_state_file_name = "finops_prev.state"
    data_file_name = ""
    storage_local_bucket = ""
    storage_remote_bucket = ""

    config = {
        "prefix": prefix,
        "data_file_format": data_file_format,
        "data_file_name": data_file_name,
        "start_ts": start_ts,
        "cloud_provider": cloud_provider,
        "granularity": granularity,
        "metrics": metrics,
        "dimensions": dimensions,
        "client_name": params["client_name"],
        "current_state_file_name": current_state_file_name,
        "previous_state_file_name": previous_state_file_name,
        "storage_local_bucket": storage_local_bucket,
        "storage_remote_bucket": storage_remote_bucket,
    }
    return config


def init_state(config):
    state = {}
    return state


def get_query_dates():
    ret_status = {"code": 200, "message": ""}
    query_start_date = ""
    query_end_date = ""
    return query_start_date, query_end_date, ret_status


def generate_cost_data(state):
    ret_status = {"code": 200, "message": ""}
    return cost_data, ret_status


def process_cost_data(data):
    ret_status = {"code": 200, "message": ""}
    return ret_status


params = {
    "sink_account_name": "finopsstorageaccount2003",
    "sink_container_name": "finopscontainer",
    "sink_account_key": "MDPnx/y+PPsv63rnYL1kiepAtc/w196OIwu1dZHrCqBI1Kjz562Ja/iUDqdk9a2zExLWaKJGKLMn+AStEaNtfg==",
    "client_name": "client1",
}

config = init_config(params)
new_state = init_state(config)
query_start_date, query_end_date, ret_status = get_query_dates(config)
if ret_status["code"] != 200:
    new_state["execution"]["status"] = "failed"
    new_state["execution"]["message"] = ret_status["message"]
else:
    new_state["params"]["start_date"] = query_start_date
    new_state["params"]["end_date"] = query_end_date

    cost_data, ret_status = generate_cost_data(new_state)
    if ret_status["code"] != 200:
        new_state["execution"]["status"] = "failed"
        new_state["execution"]["message"] = ret_status["message"]
    else:
        ret_status = process_cost_data(cost_data)
        if ret_status["code"] != 200:
            new_state["execution"]["status"] = "failed"
            new_state["execution"]["message"] = ret_status["message"]
