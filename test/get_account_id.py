import boto3


def get_aws_account_id():
    # Create an STS (Security Token Service) client
    sts_client = boto3.client("sts")

    try:
        # Get the AWS account ID from the caller's identity
        response = sts_client.get_caller_identity()
        account_id = response["Account"]

        return account_id

    except Exception as e:
        print(f"Error: {e}")
        return None


if __name__ == "__main__":
    # Get the AWS account ID
    aws_account_id = get_aws_account_id()

    if aws_account_id:
        print(f"AWS Account ID: {aws_account_id}")
    else:
        print("Unable to retrieve AWS Account ID.")
