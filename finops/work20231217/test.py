import yaml
from datetime import datetime, timedelta


class ConfigManager:
    def __init__(self, config_file) -> None:
        self.config_file = config_file
        self.config_data = self.load_config()

    def load_config(self):
        try:
            with open(self.config_file, "r") as file:
                data = yaml.safe_load(file)
        except:
            data = {}
        return data

    def get_accounts(self):
        accounts = []
        try:
            for acc in self.config_data["clients"]:
                account = {
                    "client_name": acc["client_name"],
                    "client_code": acc["client_code"],
                    "cloud_name": acc["cloud_name"],
                    "access_key_id": acc["access_key_id"],
                    "secret_access_key": acc["access_key_id"],
                }
                accounts.append(account)
        except:
            pass
        return accounts


class ContextManager:
    def __init__(self, account) -> None:
        self.start_time = datetime.now()
        self.end_time = None
        self.status = None
        self.message = None
        self.context = self.generate_context(account)

    def generate_context(self, account):
        context = {
            "credentials": {
                "access_key_id": account.get("access_key_id"),
                "secret_access_key": account.get("secret_access_key"),
            },
            "params": {
                "client_name": account.get("client_name"),
                "client_code": account.get("client_code"),
                "cloud_name": account.get("cloud_name"),
                "start_date": "",
                "end_date": "",
                "granularity": "DAILY",
                "dimensions": ["LINKED_ACCOUNT", "SERVICE"],
                "metrics": ["BlendedCost"],
                "filters": "",
            },
            "execution": {
                "start_time": self.start_time,
                "end_time": self.end_time,
                "status": self.status,
                "message": self.message,
            },
        }
        return context


config = ConfigManager("config.yaml")
for account in config.get_accounts():
    context = ContextManager(account)
    print(context.context)
