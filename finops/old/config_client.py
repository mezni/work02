import json


class Config:
    def __init__(self, config_file_path):
        self.config_file_path = config_file_path
        self.config_data = self.load_config()

    def load_config(self):
        try:
            with open(self.config_file_path, "r") as config_file:
                return json.load(config_file)
        except FileNotFoundError:
            print(f"Config file '{self.config_file_path}' not found.")
            return {}

    def get_value(self, key):
        return self.config_data.get(key)


# Example usage:
config_file_path = "config.json"  # Replace with your actual config file path
config = Config(config_file_path)

# Get a configuration value
aws_access_key = config.get_value("aws_access_key")
aws_secret_key = config.get_value("aws_secret_key")

if aws_access_key:
    print(f"AWS Access Key: {aws_access_key}")
    print(f"AWS Secret Key: {aws_secret_key}")
else:
    print("AWS credentials not found in the configuration.")
