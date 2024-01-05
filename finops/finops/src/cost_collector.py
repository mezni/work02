__author__ = "Mohamed Ali MEZNI"
__version__ = "2024-01-05"

from cost_core import Settings, ConfigManager

settings, status = Settings().get_settings()
if not status["error"]:
    print(status["message"])
else:
    pass

config = ConfigManager(settings["config_file_name"])
print(config.get_accounts())
