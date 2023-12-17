import boto3


def xxxx():
    pricing_client = boto3.client("pricing", region_name="us-east-1")
    response = pricing_client.describe_services()

    service_names = []

    for service in response["Services"]:
        print(service["ServiceCode"])


pricing_client = boto3.client(
    "pricing", region_name="us-east-1"
)  # Specify your desired AWS region

response = pricing_client.describe_services()

services_info = []


# for service in response["Services"]:
#   service_code = service["ServiceCode"]
#   service_name = service["ServiceName"]
#   print(service_code, service_name)
response = pricing_client.get_products(ServiceCode="AmazonEC2")
print(response)
