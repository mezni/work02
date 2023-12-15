import json
from dataclasses import dataclass


@dataclass
class ClientConfig:
    client_name: str
    client_code: str
    cloud_name: str
    access_key: str
    access_secret: str


client_config_data = {
    "client_name": "client test",
    "client_code": "clienttest",
    "cloud_name": "aws",
    "access_key": "",
    "access_secret": "",
}

client_config = ClientConfig(**client_config_data)
print(client_config.client_name)
