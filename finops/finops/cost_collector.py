__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-02-05"

import os, sys, logging, uuid, json, time
from datetime import datetime, timedelta, timezone
from cost_core import Settings, ConfigManager, StorageManager, VaultManager
from azure.identity import ClientSecretCredential, DefaultAzureCredential
from azure.mgmt.costmanagement import CostManagementClient
from azure.mgmt.costmanagement.models import (
    QueryDefinition,
    QueryDataset,
    QueryTimePeriod,
    QueryAggregation,
    QueryGrouping,
)


def get_logger(name):
    logger = logging.getLogger(name)
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


def state_backup(account_conf, state):
    try:
        state_error = state["execution"]["error"]
    except:
        state_error = True

    state_file_name = account_conf["last_state_file_name"]
    prev_state_file_name = account_conf["prev_state_file_name"]
    if not state_error:
        storage_mgr.download_blob(
            bronze_container,
            "logs/" + state_file_name,
            tmp_dir + "/" + state_file_name,
        )
        storage_mgr.upload_blob(
            bronze_container,
            tmp_dir + "/" + state_file_name,
            "logs/" + prev_state_file_name,
        )
        logger.info(f"   status=SUCCESS")
    else:
        logger.info(f"   status=FAIL")


def state_copy(account_conf, state):
    state_file_name = account_conf["last_state_file_name"]
    with open(tmp_dir + "/" + state_file_name, "w") as fp:
        state_str = json.dumps(state, indent=4)
        print(state_str, file=fp)

    storage_mgr.upload_blob(
        bronze_container, tmp_dir + "/" + state_file_name, "logs/" + state_file_name
    )


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
        self.client = self.create_client()

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
        secrets = self.config["secrets"]
        client_id = ""
        client_secret = ""
        tenant_id = ""
        for s in secrets:
            if "client_id" in s.keys():
                client_id = s["client_id"]
            elif "client_secret" in s.keys():
                client_secret = s["client_secret"]
            elif "tenant_id" in s.keys():
                tenant_id = s["tenant_id"]
            else:
                pass
        if client_id == "" or client_secret == "" or tenant_id == "":
            credentials = DefaultAzureCredential()
        else:
            credentials = ClientSecretCredential(
                tenant_id=tenant_id, client_id=client_id, client_secret=client_secret
            )
        client = CostManagementClient(credentials)
        return client

    def get_cost_data(self):
        cost_data = []
        dt_start = datetime.strptime(self.start_date, "%Y-%m-%d")

        epoch_start = dt_start.timestamp()
        st_time_start = time.localtime(epoch_start)
        tz_start = timezone(timedelta(seconds=st_time_start.tm_gmtoff))

        dt_end = datetime.strptime(self.end_date, "%Y-%m-%d")

        epoch_end = dt_end.timestamp()
        st_time_end = time.localtime(epoch_end)
        tz_end = timezone(timedelta(seconds=st_time_end.tm_gmtoff))

        time_period = QueryTimePeriod(
            from_property=dt_start.astimezone(tz_start),
            to=dt_end.astimezone(tz_end),
        )

        subscription_id = "1ebabb15-8364-4ada-8de3-a26abeb7ad59"
        resource_group_name = "bi-opportunite-dev-rg"
        query_aggregation = dict()
        query_aggregation["totalCost"] = QueryAggregation(
            name="Cost", function="Sum"
        )  # in result, will be column with index = 0
        query_aggregation["totalCostUSD"] = QueryAggregation(
            name="CostUSD", function="Sum"
        )  # in result, will be column with index = 1
        query_grouping = [
            QueryGrouping(type="Dimension", name="ResourceId"),
            QueryGrouping(type="Dimension", name="ChargeType"),
            QueryGrouping(type="Dimension", name="PublisherType"),
        ]

        querydataset = QueryDataset(
            granularity="Daily",
            configuration=None,
            aggregation=query_aggregation,
            grouping=query_grouping,
        )
        query = QueryDefinition(
            type="ActualCost",
            timeframe="Custom",
            time_period=time_period,
            dataset=querydataset,
        )
        scope = f"/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}"

        result = self.client.query.usage(scope=scope, parameters=query)
        for row in result.as_dict()["rows"]:
            print(row)
            cost_data.append(row)
        return cost_data

    def get_state(self):
        state = {
            "execution": {
                "context_id": self.context_id,
                "start_time": self.start_time.strftime("%d-%m-%Y %H:%M:%S"),
                "end_time": self.end_time.strftime("%d-%m-%Y %H:%M:%S"),
                "error": self.error,
                "message": self.message,
                "output_file": self.config["cost_file_name"],
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


cost_record = {
    "Client": "",
    "Date": "",
    "Provider": "",
    "SubscriptionName": "",
    "SubscriptionId": "",
    "ServiceName": "",
    "ServiceTier": "",
    "Resource": "",
    "ResourceId": "",
    "ResourceLocation": "",
    "ResourceType": "",
    "ResourceGroupName": "",
    "ResourceGroupId": "",
    "Product": "",
    "Meter": "",
    "Tags": "[]",
    "Cost": "",
    "CostUSD": "",
    "Currency": "",
}

# Main
env_file = "env"
clients_file = "clients.yaml"
tmp_dir = "/tmp"
query_history_days = 90

logger = get_logger(__name__)
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

    if account["cloud"] == "aws":
        pass
    elif account["cloud"] == "azure":
        cost_data = CostAzure(account_conf)
        state = cost_data.generate_csv()
    else:
        state = {}
        logger.info(f"   status=FAIL  cloud <{account['cloud']}> non implemente")
    state_backup(account_conf, state)
    state_copy(account_conf, state)

    logger.info("")
logger.info("Fin")
