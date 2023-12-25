import yaml, json, uuid
from datetime import datetime


class ContextManager:
    def __init__(self) -> None:
        self.context_id = str(uuid.uuid4)
        self.start_time = datetime.now()
        self.end_time = None
        self.status = "success"
        self.message = ""
        self.data = [1, 2, 3]

    def __iter__(self):
        return iter(self.data)


ctx = ContextManager()
for c in ctx:
    print(c)


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

    def get_config(self) -> list:
        return self.config_data["app"]

    def get_accounts(self) -> list:
        accounts = []
        try:
            for acc in self.config_data["clients"]:
                account = {
                    "client_name": acc["client_name"],
                    "client_code": acc["client_code"],
                    "cloud_name": acc["cloud_name"],
                    "account_name": acc["account_name"],
                    "access_key_id": acc["access_key_id"],
                    "secret_access_key": acc["secret_access_key"],
                }
                accounts.append(account)
        except:
            pass
        return accounts

    def __iter__(self):
        return iter(self.accounts)


cfg = ConfigManager("config.yaml")
for c in cfg:
    print(c)
