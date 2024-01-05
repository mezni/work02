__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-01-05"

import yaml
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    CONFIG_FILE_NAME: str
    KEY_VAULT_NAME: str
    BRONZE_CONTAINER: str
    SILVER_CONTAINER: str

    model_config = SettingsConfigDict(env_file="./.env")

    def get_settings(self) -> dict:
        status = {"error": False, "message": ""}
        try:
            settings = {
                "key_vault_name": self.KEY_VAULT_NAME,
                "bronze_container": self.BRONZE_CONTAINER,
                "silver_container": self.SILVER_CONTAINER,
                "config_file_name": self.CONFIG_FILE_NAME,
            }
        except:
            settings = {}
            status["error"] = True
            status["message"] = "get settings"
        return settings, status
