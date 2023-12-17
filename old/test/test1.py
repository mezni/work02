import boto3
from datetime import datetime, timedelta


def get_cost_by_tags(start_date, end_date, tag_keys):
    # Create a Cost Explorer client
    ce_client = boto3.client("ce")

    if 1 == 1:
        # Get the cost and usage data
        response = ce_client.get_cost_and_usage(
            TimePeriod={
                "Start": start_date.strftime("%Y-%m-%d"),
                "End": end_date.strftime("%Y-%m-%d"),
            },
            Granularity="DAILY",
            Metrics=["BlendedCost"],
            GroupBy=[{"Type": "TAG", "Key": "projet"}],
            #            GroupBy=[{"Type": "TAG", "Key": tag_key} for tag_key in tag_keys],
        )

        # Extract and print relevant information
        for result_by_time in response["ResultsByTime"]:
            for group in result_by_time["Groups"]:
                print(group)
                for tag in group["Keys"]:
                    print(tag)


#    except Exception as e:
#        print(f"Error: {e}")


if __name__ == "__main__":
    # Specify the start and end dates for the cost analysis
    start_date = datetime.now() - timedelta(days=30)
    end_date = datetime.now()

    # Specify the list of tag keys for grouping
    tag_keys = ["projet", "test"]

    # Get the cost grouped by the specified tags for the specified time period
    get_cost_by_tags(start_date, end_date, tag_keys)




            Filter={
                'Dimensions': {
                    'Key': 'LINKED_ACCOUNT',
                    'Values': [account_id]
                }
            }
    

            GroupBy=[
                {'Type': 'DIMENSION', 'Key': 'LINKED_ACCOUNT'},
                {'Type': 'DIMENSION', 'Key': 'SERVICE'}
            ]

            Filter={
                'And': [
                    {
                        'Dimensions': {
                            'Key': 'SERVICE',
                            'Values': ['AmazonS3']
                        }
                    },
                    {
                        'Dimensions': {
                            'Key': 'REGION',
                            'Values': ['us-east-1']
                        }
                    }
                ]
            }        