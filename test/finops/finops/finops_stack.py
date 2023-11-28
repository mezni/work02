from constructs import Construct
from aws_cdk import (
    Duration,
    Stack,
    aws_iam as iam,
    aws_s3 as s3,
)


class FinopsStack(Stack):
    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        account_id = Stack.of(self).account
        # Create User
        finops_user = iam.User(self, "finops_user", user_name="finops_user")
        finops_user_arn = finops_user.user_arn

        # Create Bucket
        finops_bucket = s3.Bucket(
            self, "finops_bucket", bucket_name="finops-bucket-" + str(account_id)
        )


"""
        queue = sqs.Queue(
            self, "FinopsQueue",
            visibility_timeout=Duration.seconds(300),
        )

        topic = sns.Topic(
            self, "FinopsTopic"
        )

        topic.add_subscription(subs.SqsSubscription(queue))
"""
