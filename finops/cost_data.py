from core import ConfigManager, ContextManager

__author__ = "Mohamed Ali MEZNI"
__version__ = "2023-12-15"


config = ConfigManager("config.yaml")
accounts = config.get_accounts()
for account in accounts:
    context = ContextManager(account)
    print(context.start_time)
    print(context.account_config)
