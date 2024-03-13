import random, uuid
import pandas as pd

class ResourceAzure:
    def __init__(self, config) -> None:
        self.organization_name = None
        self.owner = None
        self.project_name = None
        self.subscription_id = None
        self.resource_group_name = None
        self.tags = None
        self.resource_count = None
        self.vpc_name = None
        self.vpc_id = None
        self.region = None
        self.get_config(config)

    def get_config(self, config) -> None:
        self.organization_name = config["organization_name"]
        self.owner = config["owner"]
        self.project_name = config["project_name"]
        self.subscription_id = config["subscription_id"]
        self.resource_group_name = config["resource_group_name"]
        self.tags = config["tags"]
        self.vpc_name = config["vpc_name"]
        self.vpc_id = f"/subscriptions/{self.subscription_id}/resourcegroups/{self.resource_group_name}/providers/Microsoft.Network/virtualNetworks/{self.vpc_name}"
        self.resource_count = int(config["resource_count"])
        self.region = config["region"]

    def get_tags(self):
        tags_output = {}
        if self.tags:
            tag_list = self.tags.split("|")
            for t in tag_list:
                tags_output[t.split("=")[0]] = t.split("=")[1]
        return tags_output

    def generate_resource_info(self, service_type, resource_type):
        resource_meta = {}
        resource_name = ""
        resource_id = ""
        if resource_type == "Instance":
            resource_name = (
                "vm-" + self.project_name.lower() + "-" + str(random.randint(11, 99))
            )
            resource_id = f"/subscriptions/{self.subscription_id}/resourceGroups/{self.resource_group_name}/providers/{service_type}/virtualMachines/{resource_name}"
            resource_meta = {
                "OS": "Linux",
                "Size": "Standard_B2ms",
                "VPC id": self.vpc_id,
                "VPC name": self.vpc_name,
            }
        if resource_type == "Volume":
            resource_name = (
                "vol-"
                + self.project_name.lower()
                + "-"
                + self.region.replace(" ", "-").lower()
            )
            resource_id = f"/subscriptions/{self.subscription_id}/resourceGroups/{self.resource_group_name}/providers/{service_type}/virtualMachines/{resource_name}"
            resource_meta = {
                "Size": str(16 * random.randint(2, 9)) + " GiB",
                "Volume type": "Microsoft.Compute/disks",
                "Attached": "true",
            }
        if resource_type == "Snapshot":
            resource_name = (
                "snap-"
                + self.project_name.lower()
                + "-"
                + self.region.replace(" ", "-").lower()
                + "-release-11-1"
            )
            resource_id = f"/subscriptions/{self.subscription_id}/resourceGroups/{self.resource_group_name}/providers/{service_type}/virtualMachines/{resource_name}"
            resource_meta = {
                "State": "Succeeded",
                "Size": str(16 * random.randint(2, 9)) + " GiB",
            }
        if resource_type == "IP Address":
            resource_name = "pip-alloc-" + str(uuid.uuid4()).split("-")[-1]
            resource_id = f"/subscriptions/{self.subscription_id}/resourceGroups/{self.resource_group_name}/providers/{service_type}/virtualMachines/{resource_name}"
            resource_meta = {
                "ip": ".".join(map(str, (random.randint(0, 255) for _ in range(4))))
            }
        if resource_type == "Bucket":
            resource_name = (
                self.project_name + "-" + self.region.replace(" ", "-").lower()
            )
            resource_id = f"/subscriptions/{self.subscription_id}/resourceGroups/{self.resource_group_name}/providers/{service_type}/virtualMachines/{resource_name}"
            resource_meta = {}
        if resource_type == "SQL Storage":
            resource_name = (
                "db-" + self.project_name + "-" + self.region.replace(" ", "-").lower()
            )
            resource_id = f"/subscriptions/{self.subscription_id}/resourceGroups/{self.resource_group_name}/providers/{service_type}/virtualMachines/{resource_name}"
            resource_meta = {}
        if resource_type == "Bandwidth":
            resource_name = ""
            resource_id = f"/subscriptions/{self.subscription_id}/resourceGroups/{self.resource_group_name}/providers/{service_type}/virtualMachines/{resource_name}"
            resource_meta = {"transfer": "Intra Continent Data Transfer Out"}
        if resource_type == "Table":
            resource_name = ""
            resource_id = f"/subscriptions/{self.subscription_id}/resourceGroups/{self.resource_group_name}/providers/{service_type}/virtualMachines/{resource_name}"
            resource_meta = {"transfer": "RA-GRS Data Stored"}

        return resource_name, resource_id, resource_meta

    def generate_resources(self):
        resources = []
        vm_params = [
            ["Microsoft.Compute", "Instance"],
            ["Microsoft.Compute", "Volume"],
            ["Microsoft.Compute", "Snapshot"],
        ]
        other_params = [
            ["Microsoft.Network", "IP Address"],
            ["Microsoft.Storage", "Bucket"],
            ["Microsoft.Storage", "SQL Storage"],
            ["Microsoft.Storage", "Table"],
            ["Microsoft.Network", "Bandwidth"],
        ]
        i = self.resource_count
        while i > 0:
            if i > 5:
                for s in vm_params:
                    service_type = s[0]
                    resource_type = s[1]
                    resource_name, resource_id, resource_meta = (
                        self.generate_resource_info(service_type, resource_type)
                    )
                    resource = {
                        "organization_name": self.organization_name,
                        "owner": self.owner,
                        "provider": "azure",
                        "service_name": service_type,
                        "resource_type": resource_type,
                        "resource_name": resource_name,
                        "resource_id": resource_id,
                        "region": self.region,
                        "pool": "Azure " + self.project_name,
                        "meta": resource_meta,
                        "tags": self.get_tags(),
                    }
                    resources.append(resource)
                for j in range(2):
                    r = random.randint(0, len(other_params) - 1)
                    service_type = other_params[r][0]
                    resource_type = other_params[r][1]
                    resource_name, resource_id, resource_meta = (
                        self.generate_resource_info(service_type, resource_type)
                    )
                    resource = {
                        "organization_name": self.organization_name,
                        "owner": self.owner,
                        "provider": "azure",
                        "service_name": service_type,
                        "resource_type": resource_type,
                        "resource_name": resource_name,
                        "resource_id": resource_id,
                        "region": self.region,
                        "pool": "Azure " + self.project_name,
                        "meta": resource_meta,
                        "tags": self.get_tags(),
                    }

                    resources.append(resource)
                i = i - 5
            else:
                for j in range(i):
                    r = random.randint(0, len(other_params) - 1)
                    service_type = other_params[r][0]
                    resource_type = other_params[r][1]
                    resource_name, resource_id, resource_meta = (
                        self.generate_resource_info(service_type, resource_type)
                    )
                    resource = {
                        "organization_name": self.organization_name,
                        "owner": self.owner,
                        "provider": "azure",
                        "service_name": service_type,
                        "resource_type": resource_type,
                        "resource_name": resource_name,
                        "resource_id": resource_id,
                        "region": self.region,
                        "pool": "Azure " + self.project_name,
                        "meta": resource_meta,
                        "tags": self.get_tags(),
                    }
                    resources.append(resource)
                    i = i - 1

        return resources


df_accounts = pd.read_csv("accounts.csv")
df_accounts = df_accounts.fillna("")

accounts = df_accounts.to_dict("records")
for acc in accounts:
    if acc["provider"] == "azure":
        resource = ResourceAzure(acc)
        resources = resource.generate_resources()
        for r in resources:
            print(r)
