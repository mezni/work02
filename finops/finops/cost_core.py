__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-02-04"

import yaml
from pydantic_settings import BaseSettings, SettingsConfigDict
from datetime import datetime

env_file = "env"


class Settings(BaseSettings):
    KEY_VAULT_NAME: str
    STORAGE_ACCOUNT_NAME: str
    STORAGE_ACCOUNT_KEY: str
    WORK_CONTAINER: str
    BRONZE_CONTAINER: str
    SILVER_CONTAINER: str

    model_config = SettingsConfigDict(env_file=env_file)

    def get_settings(self) -> dict:
        try:
            settings = {
                "key_vault_name": self.KEY_VAULT_NAME,
                "storage_account_name": self.STORAGE_ACCOUNT_NAME,
                "storage_account_key": self.STORAGE_ACCOUNT_KEY,
                "work_container": self.WORK_CONTAINER,
                "bronze_container": self.BRONZE_CONTAINER,
                "silver_container": self.SILVER_CONTAINER,
            }
        except:
            settings = {}
        return settings


class ConfigManager:
    def __init__(self, config_file) -> None:
        self.config_file = config_file
        self.config_data = self.load_config()

    def load_config(self) -> dict:
        try:
            with open(self.config_file, "r") as file:
                data = yaml.safe_load(file)
        except:
            data = {}
        return data

    def get_accounts(self) -> list:
        accounts = []
        clients_data = self.config_data["clients"]

        for cli in clients_data:
            client = cli.get("name", "")
            for acc in cli["accounts"]:
                account = acc.get("name", "")
                cloud = acc.get("cloud", "")
                credentials = acc.get("credentials", {})
                client_code = client.replace(" ", "")
                account_code = account.replace("-", "_")
                file_prefix = client_code + "_" + account_code
                last_state_file_name = "state" + "_" + file_prefix + ".json"
                prev_state_file_name = "state" + "_" + file_prefix + "_prev.json"
                ts = datetime.now().strftime("%Y%m%d%H%M%S")
                cost_file_name = "finops_" + file_prefix + "_" + ts + ".csv"

                account = {
                    "client": client,
                    "account": account,
                    "cloud": cloud,
                    "credentials": credentials,
                    "last_state_file_name": last_state_file_name,
                    "prev_state_file_name": prev_state_file_name,
                    "cost_file_name": cost_file_name,
                }
                accounts.append(account)
        return accounts
