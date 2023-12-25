import yaml, json, uuid
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
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


# MAIN
settings = Settings()

config_file = "config.yaml"
storage_mgr = StorageManager()

config = ConfigManager(config_file)

for account_cfg in config.get_accounts():
    print(account_cfg)
    last_state = storage_mgr.get_content()
    prev_state = storage_mgr.get_content()
