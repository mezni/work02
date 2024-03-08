import uuid, random


def get_aws_amis():
    aws_amis = []
    for os in ["Linux", "Windows"]:
        for i in range(10):
            random_list = str(uuid.uuid4()).split("-")
            ami = "ami-" + random_list[-2] + random_list[-1]
            a = {"os": os, "ami": ami}
            aws_amis.append(a)
    return aws_amis


class ResourceAws:
    def __init__(self, config) -> None:
        self.project_name = None
        self.vpc_name = None
        self.vpc_id = None
        self.region = None
        self.init_attrs(config)

    def init_attrs(self, config):
        random_list = str(uuid.uuid4()).split("-")
        if config["tags"].get("project_name"):
            project_name = config["tags"].get("project_name", random_list[0]).lower()
            self.project_name = project_name
            self.vpc_name = "vpc-" + project_name
        else:
            self.vpc_name = ""
        self.vpc_id = "vpc-" + random_list[0]
        self.region = random.choice(aws_regions)

    def generate_resource(self):
        pass


pool = {
    "pool_name": "AWS HQ",
    "provider": "aws",
    "resources": {"vm": 2},
    "tags": {"project_name": "phenix", "env": "dev"},
}

aws_regions = ["us-east-1", "us-east-2", "us-west-1", "us-west-2"]
aws_amis = get_aws_amis()


r = ResourceAws(pool)
print(r.vpc_name, r.vpc_id, r.region)
