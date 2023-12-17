from finops.core_02 import ConfigManager, ContextManager

__author__ = "Mohamed Ali MEZNI"
__version__ = "2023-12-15"


config = ConfigManager("config.yaml")
accounts = config.get_accounts()
for account in accounts:
    context = ContextManager(account)
    context.set_end_time()
    context_json = context.get_state()
    print(context_json)
