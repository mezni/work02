import boto3


def get_aws_account_info():
    # Create an IAM client
    iam_client = boto3.client("iam")
    response = iam_client.get_account_summary()
    print(response)
    try:
        # Get information about the AWS account
        response = iam_client.get_account_summary()

        # Print relevant information
        account_id = response["SummaryMap"]["AccountSummary"]["AccountMFAEnabled"]
        account_name = response["SummaryMap"]["AccountSummary"]["AccountName"]

        print(f"AWS Account ID: {account_id}")
        print(f"AWS Account Name: {account_name}")

    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    # Get information about the AWS account
    get_aws_account_info()
