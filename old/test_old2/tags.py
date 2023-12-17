import boto3


def list_bucket_tags(bucket_name):
    # Specify your AWS credentials and region
    aws_access_key_id = "your_access_key"
    aws_secret_access_key = "your_secret_key"
    aws_region = "your_aws_region"

    # Create a Boto3 S3 client
    s3_client = boto3.client(
        "s3"
    )  # , aws_access_key_id=aws_access_key_id, aws_secret_access_key=aws_secret_access_key, region_name=aws_region)

    # Get bucket tagging configuration
    response = s3_client.get_bucket_tagging(Bucket=bucket_name)

    # Extract and print tags
    tags = response["TagSet"]
    print(f"Tags for S3 bucket {bucket_name}:")
    for tag in tags:
        print(f"{tag['Key']}: {tag['Value']}")


if __name__ == "__main__":
    # Replace 'your_access_key', 'your_secret_key', 'your_aws_region' with your actual AWS credentials and region
    # Replace 'your_s3_bucket_name' with the name of the S3 bucket you want to list tags for
    aws_bucket_name = "mybucket-projet-a"

    list_bucket_tags(aws_bucket_name)
