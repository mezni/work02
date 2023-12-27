__author__ = "Mohamed Ali MEZNI"
__version__ = "2023-12-26"

import yaml, json, uuid
from datetime import datetime
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


class VaultManager:
    def __init__(self, key_vault_name) -> None:
        pass


class StorageManager:
    def __init__(self) -> None:
        pass

    def get_content(self, file_name) -> dict:
        return {}

    def upload_data(self, data):
        pass


class ContextManager:
    def __init__(self, account, last_state, prev_state) -> None:
        self.context_id = str(uuid.uuid4())
        self.start_time = datetime.now()
        self.end_time = None
        self.status = "success"
        self.message = ""
        self.cloud_name = account["cloud_name"]
        self.credentials = self.init_credentials(account)
        self.variables = self.init_variables(account)
        self.params = self.init_params(account)

    def init_credentials(self, account):
        pass

    def init_variables(self, account):
        pass

    def init_params(self, account):
        pass

    def get_states(self):
        return {}, {}

    def get_context(self):
        context = {}
        return context

    def exit(self):
        self.end_time = datetime.now()


class CostAws:
    def __init__(self, context) -> None:
        pass


class CostAzure:
    def __init__(self, context) -> None:
        pass


# MAIN
config_file = "config.yaml"

settings = Settings()
key_vault_name = settings.KEY_VAULT_NAME
bronze_container = settings.BRONZE_CONTAINER
silver_container = settings.SILVER_CONTAINER

storage_mgr = StorageManager()
keyvault_mgr = VaultManager(key_vault_name)

config = ConfigManager(config_file)

for account_cfg in config.get_accounts():
    last_state_file = account_cfg["account_name"] + "_last.state"
    prev_state_file = account_cfg["account_name"] + "_prev.state"
    last_state = storage_mgr.get_content(last_state_file)
    prev_state = storage_mgr.get_content(prev_state_file)
    ctx = ContextManager(account_cfg, last_state, prev_state)
    if ctx.cloud_name == "aws":
        cost_data = CostAws(ctx.get_context())
    if ctx.cloud_name == "azure":
        cost_data = CostAzure(ctx.get_context())
    ctx.exit()
    storage_mgr.upload_data(cost_data)
    current_state, last_state = ctx.get_states()
    storage_mgr.upload_data(current_state)
    storage_mgr.upload_data(last_state)
