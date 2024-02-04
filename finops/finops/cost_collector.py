__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-02-05"

import os, sys, logging, uuid, json
from datetime import datetime, timedelta
from cost_core import Settings, ConfigManager, StorageManager, VaultManager


def get_logger():
    logger = logging.getLogger(__name__)
    logger.setLevel(level=logging.INFO)
    fh = logging.StreamHandler()
    fh_formatter = logging.Formatter("%(asctime)s %(levelname)s - %(message)s")
    fh.setFormatter(fh_formatter)
    logger.addHandler(fh)
    return logger


def check_file_existance(file_name):
    if not os.path.isfile(file_name):
        logger.error(f"file <{file_name}> does not exists")
        logger.info("End")
        sys.exit(1)


def get_secrets(account):
    secrets = []
    credentials = account.get("credentials", {})
    for cre in credentials:
        if cre.get("store", "") == "inline":
            sec = {cre.get("key", ""): cre.get("value", "")}
            secrets.append(sec)
        elif cre.get("store", "") == "keystore":
            secret_key = cre.get("value", "")
            secret_value = keyvault_mgr.get_secret(secret_key)
            sec = {cre.get("key", ""): secret_value}
            secrets.append(sec)
        else:
            pass

    account["secrets"] = secrets
    return account


def get_state(state_file_name):
    storage_mgr.download_blob(
        bronze_container,
        "logs/" + state_file_name,
        tmp_dir + "/" + state_file_name,
    )

    try:
        with open(tmp_dir + "/" + state_file_name) as fp:
            state = json.load(fp)
    except:
        state = {}
    return state


def get_dates(account):
    last_start_date = ""
    last_end_date = ""
    last_state = get_state(account["last_state_file_name"])
    prev_state = get_state(account["prev_state_file_name"])
    try:
        if not last_state["execution"]["error"]:
            last_start_date = last_state["params"]["start_date"]
            last_end_date = last_state["params"]["end_date"]
        else:
            last_start_date = prev_state["params"]["start_date"]
            last_end_date = prev_state["params"]["end_date"]
    except:
        pass

    account["last_start_date"] = last_start_date
    account["last_end_date"] = last_end_date
    return account


class CostAzure:
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
        return None

    def get_cost_data(self):
        cost_data = None
        return cost_data

    def get_state(self):
        state = {
            "execution": {
                "context_id": self.context_id,
                "start_time": self.start_time.strftime("%d-%m-%Y %H:%M:%S"),
                "end_time": self.end_time.strftime("%d-%m-%Y %H:%M:%S"),
                "error": self.error,
                "message": self.message,
                "output_file": self.output_file_name,
            },
            "params": {
                "start_date": self.start_date,
                "end_date": self.end_date,
                "granularity": "DAILY",
                "dimensions": ["LINKED_ACCOUNT", "SERVICE"],
                "filter": "",
            },
        }
        return state

    def generate_csv(self):
        cost_data = self.get_cost_data()
        self.end_time = datetime.now()
        state = self.get_state()
        return state


# Main
env_file = "env"
clients_file = "clients.yaml"
tmp_dir = "/tmp"
query_history_days = 90

logger = get_logger()
logger.info("Debut")
check_file_existance(env_file)
check_file_existance(clients_file)
settings = Settings().get_settings()
if not settings:
    logger.error(f"cannot read settings")
    logger.info("End")
    sys.exit(1)

config = ConfigManager(clients_file)
if not config:
    logger.error(f"cannot read clients config")
    logger.info("End")
    sys.exit(1)

bronze_container = settings["bronze_container"]
silver_container = settings["silver_container"]
storage_mgr = StorageManager(
    settings["storage_account_name"], settings["storage_account_key"]
)

keyvault_mgr = VaultManager(settings["key_vault_name"])

accounts = config.get_accounts()
for account in accounts:
    logger.info(
        f"-> traitement client=<{account['client']}>  account=<{account['account']}>"
    )
    account_conf = get_secrets(account)
    account_conf = get_dates(account_conf)
    print(account_conf)
    if account["cloud"] == "aws":
        logger.info(f"   status=SUCCESS")
    elif account["cloud"] == "azure":
        cost_data = CostAzure(account_conf)
        state = cost_data.generate_csv()
        print(state)
        logger.info(f"   status=SUCCESS")
    else:
        logger.info(f"   status=FAIL  cloud <{account['cloud']}> non implemente")
    logger.info("")
logger.info("Fin")
