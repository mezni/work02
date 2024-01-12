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
