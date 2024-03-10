import uuid, random


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
                    print(resource)
        return resources


pool = {
    "pool_name": "AWS HQ",
    "provider": "aws",
    "resources": {"vm": 2, "other": 3},
    "tags": {"project_name": "phenix", "env": "dev"},
}

r = ResourceAws(pool)
resource_list = r.get_resources()
