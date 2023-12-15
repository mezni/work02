import yaml, random
from datetime import datetime, timedelta


class ContextManager:
    def __init__(self, account_config) -> None:
        self.start_time = datetime.now()
        self.end_time = ""
        self.account_config = account_config


class ConfigManager:
    def __init__(self, config_file) -> None:
        self.config_file = config_file
        self.config_data = self.load_config()
        self.accounts = self.get_accounts()

    def load_config(self) -> dict:
        try:
            with open(self.config_file, "r") as file:
                data = yaml.safe_load(file)
        except:
            data = {}
        return data

    def get_accounts(self) -> list:
        accounts = []
        try:
            for acc in self.config_data["clients"]:
                account = {"client_name": acc["client_name"]}
                accounts.append(account)
        except:
            pass
        return accounts
