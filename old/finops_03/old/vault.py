from azure.identity import DefaultAzureCredential
from azure.keyvault.secrets import SecretClient
from azure.core.exceptions import ResourceNotFoundError


class AzureKeyVault:
    def __init__(self, vault_url):
        self.vault_url = vault_url
        self.credential = DefaultAzureCredential()
        self.secret_client = SecretClient(
            vault_url=self.vault_url, credential=self.credential
        )

    def get_secret(self, secret_name):
        try:
            secret = self.secret_client.get_secret(secret_name)
            return secret.value
        except ResourceNotFoundError:
            print(f"Secret '{secret_name}' not found in Azure Key Vault.")
            return None


# Example usage:
vault_url = "https://your-key-vault-name.vault.azure.net/"
key_vault = AzureKeyVault(vault_url)

# Replace "your-secret-name" with the name of the secret you want to retrieve
secret_name = "your-secret-name"
secret_value = key_vault.get_secret(secret_name)

if secret_value is not None:
    print(f"Secret value for '{secret_name}': {secret_value}")
