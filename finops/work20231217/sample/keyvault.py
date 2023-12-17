from azure.identity import DefaultAzureCredential
from azure.keyvault.secrets import SecretClient


class AzureKeyVault:
    def __init__(self, vault_url):
        credential = DefaultAzureCredential()
        self.secret_client = SecretClient(vault_url=vault_url, credential=credential)

    def get_secret(self, secret_name):
        secret = self.secret_client.get_secret(secret_name)
        return secret.value

    def set_secret(self, secret_name, secret_value):
        self.secret_client.set_secret(secret_name, secret_value)

    def list_secrets(self):
        secrets = self.secret_client.list_properties()
        return [secret.name for secret in secrets]

    def delete_secret(self, secret_name):
        self.secret_client.begin_delete_secret(secret_name).wait()


# Example usage:

# Replace 'your_vault_url' with your actual Key Vault URL
vault_url = "https://your-key-vault-name.vault.azure.net"
secret_name = "example_secret"
secret_value = "super_secret_value"

# Create an instance of AzureKeyVault
azure_key_vault = AzureKeyVault(vault_url)

# Set a secret
azure_key_vault.set_secret(secret_name, secret_value)

# Get and print the secret value
retrieved_secret = azure_key_vault.get_secret(secret_name)
print("Retrieved Secret:", retrieved_secret)

# List secrets in the vault
secrets_list = azure_key_vault.list_secrets()
print("Secrets in the vault:", secrets_list)

# Delete the secret
azure_key_vault.delete_secret(secret_name)
