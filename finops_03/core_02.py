import yaml, random
from datetime import datetime, timedelta


class ContextManager:
    def __init__(self, account_config) -> None:
        self.start_time = datetime.now()
        self.end_time = ""
        self.status = "success"
        self.message = ""
        self.check_config(account_config)

    def check_config(self, account_config) -> None:
        for attr in ["client_name", "client_code", "cloud_name"]:
            if account_config.get(attr):
                setattr(self, attr, account_config.get(attr))
            else:
                self.status = "failed"
                self.message = "attribute " + attr + " is mandatory, in config file"

    def set_end_time(self) -> None:
        self.end_time = datetime.now()

    def get_state(self) -> dict:
        state = {
            "execution": {
                "start_time": self.start_time.strftime("%d-%m-%Y %H:%M:%S"),
                "end_time": self.end_time.strftime("%d-%m-%Y %H:%M:%S"),
                "status": self.status,
                "message": self.message,
            },
            "query_params": {
                "start_date": "",
                "end_date": "",
                "granurality": "",
                "dimensions": "",
            },
        }
        return state


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
                account = {
                    "client_name": acc["client_name"],
                    "client_code": acc["client_code"],
                    "cloud_name": acc["cloud_name"],
                }
                accounts.append(account)
        except:
            pass
        return accounts
