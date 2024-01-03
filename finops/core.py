__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-01-03"

import yaml
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    KEY_VAULT_NAME: str
    BRONZE_CONTAINER: str
    SILVER_CONTAINER: str

    model_config = SettingsConfigDict(env_file="./.env")


class ConfigManager:
    def __init__(self, config_file) -> None:
        self.status = "success"
        self.message = ""
        self.config_file = config_file
        self.config_data = self.load_config()

    def load_config(self) -> dict:
        try:
            with open(self.config_file, "r") as file:
                data = yaml.safe_load(file)
        except:
            self.status = "failed"
            self.message = "cannot read " + config_file
            data = {}
        return data

    def get_accounts(self) -> list:
        accounts = []
        try:
            for acc in self.config_data["clients"]:
                account = {
                    "client_name": acc["client_name"],
                    "cloud_name": acc["cloud_name"],
                    "account_name": acc["account_name"],
                    "access_key_id": acc["access_key_id"],
                    "secret_access_key": acc["secret_access_key"],
                }
                accounts.append(account)
        except:
            pass
        return accounts
