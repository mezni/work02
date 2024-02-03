__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-02-04"

import os, sys, logging
from cost_core import Settings, ConfigManager


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


# Main
env_file = "env"
clients_file = "clients.yaml"


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

accounts = config.get_accounts()
for account in accounts:
    logger.info(
        f"> traitement client=<{account['client']}>  account=<{account['account']}>"
    )
    if account["cloud"] == "aws":
        logger.info(f"  status=SUCCESS")
    elif account["cloud"] == "azure":
        logger.info(f"  status=SUCCESS")
    else:
        logger.info(f"  status=FAIL  cloud <{account['cloud']}> non implemente")
    logger.info("")
logger.info("Fin")
