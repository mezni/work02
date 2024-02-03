__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-02-04"

from pydantic_settings import BaseSettings, SettingsConfigDict

env_file = "env"


class Settings(BaseSettings):
    CONFIG_FILE_NAME: str
    KEY_VAULT_NAME: str
    STORAGE_ACCOUNT_NAME: str
    STORAGE_ACCOUNT_KEY: str
    BRONZE_CONTAINER: str
    SILVER_CONTAINER: str

    model_config = SettingsConfigDict(env_file=env_file)
