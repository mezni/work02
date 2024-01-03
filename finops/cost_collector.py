__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-01-03"

from core import Settings, ConfigManager

# MAIN
config_file = "config.yaml"

settings = Settings()
key_vault_name = settings.KEY_VAULT_NAME
bronze_container = settings.BRONZE_CONTAINER
silver_container = settings.SILVER_CONTAINER

config = ConfigManager(config_file)

print(key_vault_name)
