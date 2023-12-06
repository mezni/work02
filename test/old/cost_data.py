import boto3


org_client = boto3.client("organizations")
accounts = []
try:
    response = org_client.list_accounts()
    accounts = response["Accounts"]
except Exception as e:
    pass

if not accounts:
    session = boto3.session.Session()
    iam_client = session.client("iam")

    response = iam_client.get_user()
    account_id = response["User"]["Arn"].split(":")[4]
#    print(response)


if not accounts:
    sts_client = session.client("sts")

    response = sts_client.get_caller_identity()
    print(response)
