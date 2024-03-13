import uuid, random
import pandas as pd


class ResourceGcp:
    def __init__(self) -> None:
        pass

    def get_resources(self) -> list:
        pass


class ResourceAzure:
    def __init__(self) -> None:
        pass

    def get_resources(self) -> list:
        pass


class ResourceAws:
    def __init__(self, config) -> None:
        self.config = config
        self.amis = self.get_amis()
        self.regions = ["us-east-1", "us-east-2", "us-west-1", "us-west-2"]
        self.region = None
        self.vpc_id = None
        self.vpc_name = None
        self.set_env()

    def set_env(self) -> None:
        self.region = random.choice(self.regions)
        random_list = str(uuid.uuid4()).split("-")
        pool = self.config["tags"]
        self.vpc_id = "vpc-" + random_list[0]
        self.vpc_name = "vpc-" + (pool.get("project_name", random_list[0]).lower())

    def get_amis(self) -> list:
        aws_amis = []
        aws_sizes = ["t2.medium", "t3.medium"]
        for os in ["Linux", "Windows"]:
            for i in range(10):
                random_list = str(uuid.uuid4()).split("-")
                ami = "ami-" + random_list[-2] + random_list[-1]
                a = {"os": os, "ami": ami, "size": random.choice(aws_sizes)}
                aws_amis.append(a)
        return aws_amis

    def get_resource_info(self, resource_type, service):
        resource_id = ""
        resource_name = ""
        meta = {}
        tags = {}
        if resource_type == "Instance":
            random_list = str(uuid.uuid4()).split("-")
            resource_id = "i-" + random_list[-2] + random_list[-1]
            resource_name = (
                "vm-"
                + (
                    self.config["tags"].get("project_name", random_list[0]).lower()
                    + self.config["tags"].get("env", random_list[2]).lower()
                )
                + "-"
                + str(random_list[1])
            )
            ami = random.choice(self.amis)
            meta = {
                "Image id": ami["ami"],
                "OS": ami["os"],
                "Size": ami["size"],
                "Preinstalled": "NA",
                "VPC id": self.vpc_id,
                "VPC name": self.vpc_name,
            }
            tags = self.config["tags"]
        if resource_type == "Snapshot":
            random_list = str(uuid.uuid4()).split("-")
            resource_id = "snap-" + random_list[-2] + random_list[-1]
            resource_name = (
                (
                    self.config["tags"].get("project_name", random_list[0]).lower()
                    + self.config["tags"].get("env", random_list[2]).lower()
                )
                + "-"
                + str(random_list[0])
            )
            random_list = str(uuid.uuid4()).split("-")
            volume_id = "vol-" + random_list[-2] + random_list[-1]
            meta = {
                "State": "completed",
                "Size": str(10 * random.randint(1, 5)) + " GiB",
                "Volume id": volume_id,
            }
            tags = self.config["tags"]
        if resource_type == "Volume":
            random_list = str(uuid.uuid4()).split("-")
            resource_id = "vol-" + random_list[-2] + random_list[-1]
            resource_name = (
                (
                    self.config["tags"].get("project_name", random_list[0]).lower()
                    + self.config["tags"].get("env", random_list[2]).lower()
                )
                + "-"
                + str(random_list[0])
            )
            meta = {
                "Size": str(16 * random.randint(1, 10)) + " GiB",
                "Volume type": "gp2",
                "Attached": "false",
            }
            tags = self.config["tags"]
        if resource_type == "IP Address":
            random_list = str(uuid.uuid4()).split("-")
            resource_id = "eip-" + random_list[-2] + random_list[-1]
            resource_name = ""
            meta = {
                "ip": ".".join(map(str, (random.randint(0, 255) for _ in range(4))))
            }
            tags = self.config["tags"]
        if resource_type == "Bucket":
            random_list = str(uuid.uuid4()).split("-")
            resource_id = (
                (
                    self.config["tags"].get("project_name", random_list[0]).lower()
                    + self.config["tags"].get("env", random_list[2]).lower()
                )
                + "-"
                + str(random_list[0])
            )
            resource_name = resource_id
            meta = {}
            tags = self.config["tags"]
        resource = {
            "resource_id": resource_id,
            "resource_name": resource_name,
            "resource_type": resource_type,
            "service": service,
            "region": self.region,
            "pool": self.config["pool_name"],
            "meta": meta,
            "tags": tags,
        }
        if resource_type == "NAT Gateway":
            random_list = str(uuid.uuid4()).split("-")
            resource_id = "nat-" + random_list[-2] + random_list[-1]
            resource_name = (
                (
                    self.config["tags"].get("project_name", random_list[0]).lower()
                    + self.config["tags"].get("env", random_list[2]).lower()
                )
                + "-"
                + str(random_list[0])
            )
            meta = {}
            tags = {}
        if resource_type == "API Request":
            random_list = str(uuid.uuid4()).split("-")
            resource_id = "AmazonCloudWatch-" + random_list[-2] + random_list[-1]
            resource_name = "-"
            meta = {}
            tags = {}
        if resource_type == "Encryption Key":
            random_list = str(uuid.uuid4()).split("-")
            resource_id = random_list[-2] + random_list[-1]
            resource_name = "-"
            meta = {}
            tags = {}
        if resource_type == "Load Balancer":
            random_list = str(uuid.uuid4()).split("-")
            resource_id = random_list[-2] + random_list[-1]
            resource_name = "-"
            meta = {}
            tags = {}
        resource = {
            "organization_name": self.config["organization_name"],
            "owner": self.config["owner"],
            "provider": "aws",
            "region": self.region,
            "pool": self.config["pool_name"],
            "resource_id": resource_id,
            "resource_name": resource_name,
            "resource_type": resource_type,
            "service": service,
            "meta": meta,
            "tags": tags,
        }

        return resource

    def get_resources(self) -> list:
        resources = []
        pool = self.config["resources"]
        for k, v in pool.items():
            if k == "vm":
                for i in range(int(v)):
                    resource_type = "Instance"
                    service = "AmazonEC2"
                    resource = self.get_resource_info(resource_type, service)
                    resources.append(resource)
                    resource_type = "Snapshot"
                    service = "AmazonEC2"
                    resource = self.get_resource_info(resource_type, service)
                    resources.append(resource)
                    resource_type = "Volume"
                    service = "AmazonEC2"
                    resource = self.get_resource_info(resource_type, service)
                    resources.append(resource)
                    if random.randint(0, 1) == 0:
                        resource_type = "IP Address"
                        service = "AmazonVPC"
                        resource = self.get_resource_info(resource_type, service)
                        resources.append(resource)
            if k == "bucket":
                for i in range(int(v)):
                    resource_type = "Bucket"
                    service = "AmazonS3"
                    resource = self.get_resource_info(resource_type, service)
                    resources.append(resource)
            if k == "other":
                for i in range(int(v)):
                    resource_types = [
                        {"resource_type": "NAT Gateway", "service": "cc"},
                        {"resource_type": "API Request", "service": "AmazonCloudWatch"},
                        {"resource_type": "Encryption Key", "service": "awskms"},
                        {"resource_type": "Load Balancer", "service": "AmazonELB"},
                    ]
                    c = random.choice(resource_types)
                    resource_type = c["resource_type"]
                    service = c["service"]
                    resource = self.get_resource_info(resource_type, service)
                    resources.append(resource)
        return resources


pool = {
    "organization_name": "ByteGenius Solutions",
    "owner": "Ava Jones",
    "provider": "aws",
    "pool_name": "AWS HQ",
    "resources": {"vm": 2, "bucket": 2, "other": 3},
    "tags": {"project_name": "phenix", "env": "dev"},
}
resources = []
if pool["provider"] == "aws":
    r = ResourceAws(pool)
    rl = r.get_resources()
    resources = resources + rl
if pool["provider"] == "azure":
    r = ResourceAzure(pool)
    rl = r.get_resources()
    resources = resources + rl
if pool["provider"] == "gcp":
    r = ResourceGcp(pool)
    rl = r.get_resources()
    resources = resources + rl

output = []
for resource in resources:
    tags = []
    for k, v in resource["tags"].items():
        tags.append(k + "=" + v)
    resource["tags"] = "|".join(tags)
    meta = []
    for k, v in resource["meta"].items():
        tags.append(k + "=" + v)
    resource["meta"] = "|".join(meta)
    output.append(resource)
df_output = pd.DataFrame(output)
df_output.to_csv("test.csv", index=False)
