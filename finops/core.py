import boto3

# subscription_id = 'your_subscription_id'
# client_id = 'your_client_id'
# client_secret = 'your_client_secret'
# tenant_id = 'your_tenant_id'


class CostAws:
    def __init__(self, credentials, params) -> None:
        self.credentials = credentials
        self.params = params
        self.status = "success"
        self.message = ""

    def create_connection(self):
        for attr in ["access_key_id", "secret_access_key", "region"]:
            if self.credentials.get(attr):
                setattr(self, attr, self.credentials.get(attr))
            else:
                self.status = "failed"
                self.message = (
                    "create connection : "
                    + "attribute "
                    + attr
                    + " is mandatory, in config file"
                )
        if self.status == "failed":
            return boto3.client(
                "ce",
                aws_access_key_id=self.access_key_id,
                aws_secret_access_key=self.secret_access_key,
                region_name=self.region,
            )
        else:
            return None

    def get_cost_data(self):
        client_ce = self.create_connection()
