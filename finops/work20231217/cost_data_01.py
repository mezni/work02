import yaml
from datetime import datetime, timedelta

from azure.identity import DefaultAzureCredential
from azure.keyvault.secrets import SecretClient


class VaultManager:
    def __init__(self, key_vault_name) -> None:
        keyvault_url = f"https://{key_vault_name}.vault.azure.net"
        self.status = ""
        self.message = ""
        self.keyvault_url = keyvault_url
        self.credentials = DefaultAzureCredential()
        self.secret_client = self.create_secret_client()

    def create_secret_client(self):
        return SecretClient(vault_url=self.keyvault_url, credential=self.credentials)

    def get_secret(self, secret_name):
        try:
            secret = self.secret_client.get_secret(secret_name)
            return secret.value
        except Exception as e:
            self.status = "failed"
            self.message = e
            return None


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


class ContextManager:
    def __init__(self, account) -> None:
        self.start_time = ""
        self.end_time = ""
        self.status = ""
        self.message = ""
        self.context = self.init_context()

    def init_context(self):
        context = {
            "credentials": {
                "access_key_id": "",
                "secret_access_value": "",
                "region": "",
            },
            "params": {
                "start_date": "",
                "end_date": "",
                "client_name": "",
                "granularity": "DAILY",
                "dimensions": ["LINKED_ACCOUNT", "SERVICE"],
                "metrics": ["BlendedCost"],
                "filters": "",
            },
        }
        return context


app_config = ConfigManager("config.yaml")
# key_vault_name = app_config.get_config()["key-vault-name"]
# key_vault = VaultManager(key_vault_name)
accounts = app_config.get_accounts()
for account in accounts:
    print(account)
    #    account["secret_access_value"] = key_vault.get_secret(account["secret_access_key"])
    context = ContextManager(account)
    print(context.context)
#    if account["cloud_name"] == "aws":
