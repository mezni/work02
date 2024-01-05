__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-01-05"

import logging, sys
from cost_core import Settings, ConfigManager

logging.basicConfig(format="%(asctime)s - %(message)s", level=logging.INFO)
logging.info("Start")

settings, status = Settings().get_settings()
if status["error"]:
    logging.error(status["message"])
    logging.info("End")
    sys.exit(1)

config = ConfigManager(settings["config_file_name"])
accounts, status = config.get_accounts()
if status["error"]:
    logging.error(status["message"])
    logging.info("End")
    sys.exit(1)

print(accounts)

logging.info("End")
sys.exit(0)
