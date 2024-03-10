import uuid, random


class ResourceAws:
    def __init__(self, config) -> None:
        pass


def get_aws_amis():
    aws_amis = []
    aws_sizes = ["t2.medium", "t3.medium"]
    for os in ["Linux", "Windows"]:
        for i in range(10):
            random_list = str(uuid.uuid4()).split("-")
            ami = "ami-" + random_list[-2] + random_list[-1]
            a = {"os": os, "ami": ami, "size": random.choice(aws_sizes)}
            aws_amis.append(a)
    return aws_amis


def get_resources_config(pool):
    resources = []
    region = random.choice(aws_regions)
    random_list = str(uuid.uuid4()).split("-")
    vpc_id = "vpc-" + random_list[0]
    vpc_name = "vpc-" + (pool["tags"].get("project_name", random_list[0]).lower())

    for k, v in pool["resources"].items():
        if k == "vm":
            for i in range(int(v)):
                resource_type = "Instance"
                service = "AmazonEC2"
                random_list = str(uuid.uuid4()).split("-")
                resource_id = "i-" + random_list[-2] + random_list[-1]
                resource_name = (
                    "vm-"
                    + (
                        pool["tags"].get("project_name", random_list[0]).lower()
                        + pool["tags"].get("env", random_list[2]).lower()
                    )
                    + str(i)
                )
                ami = random.choice(aws_amis)
                meta = {
                    "Image id": ami["ami"],
                    "OS": ami["os"],
                    "Size": ami["size"],
                    "Preinstalled": "NA",
                    "VPC id": vpc_id,
                    "VPC name": vpc_name,
                }
                tags = pool["tags"]
                resource = {
                    "resource_id": resource_id,
                    "resource_name": resource_name,
                    "resource_type": resource_type,
                    "service": service,
                    "region": region,
                    "pool": pool["pool_name"],
                    "meta": meta,
                    "tags": tags,
                }
                print(resource)
        if k == "other":
            for i in range(int(v)):
                resource = {"resource_id": ""}

    return resources


aws_regions = ["us-east-1", "us-east-2", "us-west-1", "us-west-2"]
aws_amis = get_aws_amis()

pool = {
    "pool_name": "AWS HQ",
    "provider": "aws",
    "resources": {"vm": 2, "other": 3},
    "tags": {"project_name": "phenix", "env": "dev"},
}

x = get_resources_config(pool)
