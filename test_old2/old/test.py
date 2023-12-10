import boto3


def get_aws_account_alias():
    # Create an AWS Identity and Access Management (IAM) client
    iam_client = boto3.client("iam")
    #    org_client = boto3.client("organizations")
    #    response = org_client.list_accounts()
    session = boto3.session.Session()

    # Get the AWS account profile name
    profile_name = session.profile_name
    print(profile_name)
    try:
        # Get the account alias
        response = iam_client.list_account_aliases()
        print(response)
        account_alias = (
            response["AccountAliases"][0] if "AccountAliases" in response else None
        )

        return account_alias

    except Exception as e:
        print(f"Error: {e}")
        return None


if __name__ == "__main__":
    aws_account_alias = get_aws_account_alias()

    if aws_account_alias:
        print(f"AWS Account Alias (Name): {aws_account_alias}")
    else:
        print("Unable to retrieve AWS Account Alias.")
