import yaml, random
from datetime import datetime, timedelta


class Account:
    def __init__(self) -> None:
        pass


class ConfigManager:
    def __init__(self, config_file) -> None:
        self.config_file = config_file
        self.config_data = self.load_config()
        self.accounts = self.get_accounts()

    def load_config(self):
        try:
            with open(self.config_file, "r") as file:
                data = yaml.safe_load(file)
        except:
            data = {}
        return data

    def get_accounts1(self):
        accounts = []
        try:
            for acc in self.config_data["clients"]:
                account = Account()
                accounts.append(account)
        except:
            pass
        return accounts

    def get_accounts(self):
        accounts = []
        try:
            for acc in self.config_data["clients"]:
                account = {"client_name": acc["client_name"]}
                accounts.append(account)
        except:
            pass
        return accounts
