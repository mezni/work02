__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-01-12"

import sys, logging
import pandas as pd
from cost_core import Settings, ConfigManager, StorageManager

# MAIN
finops_file_name = "finops.csv"
finops_file_new_name = "finops_new.csv"

duplicate_cols = [
    "Client",
    "Date",
    "Provider",
    "SubscriptionName",
    "SubscriptionId",
    "ServiceName",
]

tmp_dir = "/tmp"

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

storage_mgr = StorageManager(
    settings["storage_account_name"], settings["storage_account_key"]
)

config = ConfigManager(settings["config_file_name"])
accounts, status = config.get_accounts()
if status["error"]:
    logger.error(status["message"])
    logger.info("End")
    sys.exit(1)


first_load = True
df_result = None
input_files = storage_mgr.list_blobs(bronze_container)
for file_name in input_files:
    if file_name.startswith("finops"):
        storage_mgr.download_blob(
            bronze_container,
            file_name,
            tmp_dir + "/" + file_name,
        )
        df = pd.read_csv(tmp_dir + "/" + file_name)
        if first_load:
            df_result = df
            first_load = False
        else:
            df_result = pd.concat([df_result, df])
if not df_result.empty:
    df_result = df_result.drop_duplicates()

    storage_mgr.download_blob(
        silver_container,
        finops_file_name,
        tmp_dir + "/" + finops_file_name,
    )
    try:
        df_finops = pd.read_csv(tmp_dir + "/" + finops_file_name)
        df_finops = pd.concat([df_finops, df_result])
    except:
        df_finops = df_result

    df_finops = df_finops.drop_duplicates(subset=duplicate_cols, keep="last")
    df_finops.to_csv(tmp_dir + "/" + finops_file_new_name, index=False)
    storage_mgr.upload_blob(silver_container, finops_file_new_name, finops_file_name)

    for file_name in input_files:
        if file_name.startswith("finops"):
            storage_mgr.delete_blob(bronze_container, file_name)

logger.info("End")
