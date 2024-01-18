__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-01-18"

import boto3
import os, sys, json, uuid, logging
from datetime import datetime, timedelta
from cost_core import Settings, ConfigManager, VaultManager, StorageManager


class CostAzure:
    def __init__(self, config) -> None:
        pass

    def create_client(self):
        pass

    def get_cost_data(self):
        pass

    def generate_csv(self):
        pass

    def construct_params(self):
        pass

    def get_state(self):
        pass


class CostAws:
    def __init__(self, config) -> None:
        self.start_time = datetime.now()
        self.end_time = ""
        self.context_id = str(uuid.uuid4())
        self.error = False
        self.message = ""
        self.config = config
        self.end_date = datetime.now().strftime("%Y-%m-%d")
        self.start_date = self.get_query_start_date()
        self.output_file_name = ""
        self.ce_client = self.create_client()

    def get_query_start_date(self):
        last_end_date = self.config["last_end_date"]
        if last_end_date == "":
            date_history = (
                datetime.utcnow() - timedelta(days=query_history_days)
            ).strftime("%Y-%m-%d")
            start_date = date_history[:-2] + "01"
        else:
            start_date = last_end_date
        if start_date[5:7] != self.end_date[5:7]:
            start_date = start_date[:-2] + "01"
        return start_date

    def create_client(self):
        try:
            return boto3.client(
                "ce",
                aws_access_key_id=self.config["access_key_id"],
                aws_secret_access_key=self.config["secret_access_key"],
                region_name=self.config["region"],
            )
        except Exception as e:
            self.error = True
            self.message = "Cannot create AWS client"
            self.end_time = datetime.now()
            return None

    def get_cost_data(self):
        pass

    def generate_csv(self):
        pass

    def construct_params(self):
        pass

    def get_state(self):
        pass


def get_state_file(state_file_name):
    storage_mgr.download_blob(
        bronze_container,
        log_dir + state_file_name,
        tmp_dir + "/" + state_file_name,
    )

    try:
        with open(tmp_dir + "/" + state_file_name) as fp:
            state = json.load(fp)
    except:
        state = {}
    return state


def get_query_dates(state_file_name, prev_state_file_name):
    last_start_date = ""
    last_end_date = ""

    last_state = get_state_file(state_file_name)
    prev_state = get_state_file(prev_state_file_name)
    try:
        if not last_state["execution"]["error"]:
            last_start_date = last_state["params"]["start_date"]
            last_end_date = last_state["params"]["end_date"]
        else:
            last_start_date = prev_state["params"]["start_date"]
            last_end_date = prev_state["params"]["end_date"]
    except:
        pass
    return last_start_date, last_end_date


def get_account_config(account):
    try:
        secret_access_key = keyvault_mgr.get_secret(account["secret_access_key_name"])
    except:
        secret_access_key = None

    client_code = account["client_name"].replace(" ", "")
    prefix = client_code + "_" + account["cloud_name"] + "_" + account["account_name"]
    state_file_name = "state_" + prefix + ".json"
    prev_state_file_name = "state_" + prefix + "_prev.json"
    last_start_date, last_end_date = get_query_dates(
        state_file_name, prev_state_file_name
    )
    config = {
        "client_name": account["client_name"],
        "client_code": client_code,
        "account_name": account["account_name"],
        "cloud_name": account["cloud_name"],
        "region": account["region"],
        "access_key_id": account["access_key_id"],
        "secret_access_key": secret_access_key,
        "last_start_date": last_start_date,
        "last_end_date": last_end_date,
        "state_file_name": state_file_name,
        "prev_state_file_name": prev_state_file_name,
        "cost_file_name": "finops_"
        + prefix
        + "_"
        + datetime.now().strftime("%Y%m%d%H%M%S")
        + ".csv",
    }
    return config


## MAIN
query_history_days = 180
tmp_dir = "/tmp"
log_dir = "logs"

logger = logging.getLogger(__name__)
logger.setLevel(level=logging.INFO)
fh = logging.StreamHandler()
fh_formatter = logging.Formatter("%(asctime)s %(levelname)s - %(message)s")
fh.setFormatter(fh_formatter)
logger.addHandler(fh)

logger.info("Start")

settings, status = Settings().get_settings()
if status["error"]:
    logger.error(status["message"])
    logger.info("End")
    sys.exit(1)
bronze_container = settings["bronze_container"]
silver_container = settings["silver_container"]

settings, status = Settings().get_settings()
if status["error"]:
    logger.error(status["message"])
    logger.info("End")
    sys.exit(1)
bronze_container = settings["bronze_container"]
silver_container = settings["silver_container"]

try:
    storage_mgr = StorageManager(
        settings["storage_account_name"], settings["storage_account_key"]
    )
except:
    logger.error("Problem to get storage manager")
    logger.info("End")
    sys.exit(1)

try:
    keyvault_mgr = VaultManager(settings["key_vault_name"])
except:
    logger.error("Problem to get storage manager")
    logger.info("End")
    sys.exit(1)

config = ConfigManager(settings["config_file_name"])
accounts, status = config.get_accounts()
if status["error"]:
    logger.error(status["message"])
    logger.info("End")
    sys.exit(1)

for account in accounts:
    logger.info(f"generate cost for : client=<{account['client_name']}>")
    if account["cloud_name"] == "aws":
        account_conf = get_account_config(account)
        cost_aws = CostAws(account_conf)
        state = cost_aws.generate_csv()
    elif account["cloud_name"] == "azure":
        account_conf = get_account_config(account)
        cost_azure = CostAzure(account_conf)
        state = cost_azure.generate_csv()
    else:
        logger.info(f"cloud=<account['cloud_name']> is not defined")
logger.info("End")
