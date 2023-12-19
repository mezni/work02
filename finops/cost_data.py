from azure.identity import DefaultAzureCredential
from azure.keyvault.secrets import SecretClient


class VaultManager:
    def __init__(self) -> None:
        keyvault_url = f"https://{keyvault_name}.vault.azure.net"
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


keyvault_name = "finops-keyvault5"
key_vault = VaultManager()
# secret = key_vault.get_secret("secret-test1")
