import yaml, json, uuid
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    KEY_VAULT_NAME: str
    BRONZE_CONTAINER: str
    SILVER_CONTAINER: str

    model_config = SettingsConfigDict(env_file="./.env")


class ConfigManager:
    def __init__(self, config_file) -> None:
        pass

    def get_accounts(self) -> list:
        accounts = [1]
        return accounts


class StorageManager:
    def __init__(self) -> None:
        pass


class StorageManager:
    def __init__(self) -> None:
        pass

    def get_content(self):
        return {}

    def upload_data(self, data):
        pass


class ContextManager:
    def __init__(self, account, last_state, prev_state) -> None:
        self.cloud_name = "aws"
        pass

    def get_states(self):
        return {}, {}

    def get_context(self):
        return {}

    def exit(self):
        pass


class CostAws:
    def __init__(self, context) -> None:
        pass


class CostAzure:
    def __init__(self, context) -> None:
        pass


# MAIN
settings = Settings()

config_file = "config.yaml"
storage_mgr = StorageManager()

config = ConfigManager(config_file)

for account_cfg in config.get_accounts():
    print(account_cfg)
    last_state = storage_mgr.get_content()
    prev_state = storage_mgr.get_content()
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
