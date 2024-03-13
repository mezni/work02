import uuid, random


regions_azure = [
    "East US",
    "East US 2",
    "West US",
    "West US 2",
    "East Europe",
    "West Europe",
    "France Central",
    "Germany West Central",
    "Norway East",
]

project_names = [
    "metalabs",
    "innovatex",
    "technest",
    "futureforge",
    "codecrafter",
    "agilewind",
    "pulsetech",
    "fusionedge",
    "nexus",
    "cybersap",
]
project_types = ["dev", "qa", "prod", "test", "sales", "marketing", "hr", "it"]

config = {
    "project_name": random.choice(project_names),
    "project_type": random.choice(project_types),
    "subscription_id": str(uuid.uuid4()),
    "region": random.choice(regions_azure),
    "resources": [{"resource": "vm", "count": 3}, {"resource": "ip", "count": 1}],
}


def create_azure_resource(config):
    resources = []
    project_name = config["project_name"]
    project_type = config["project_type"]
    subscription_id = config["subscription_id"]
    region = config["region"]
    resource_group_name = project_name + "-rg"
    for rs in config["resources"]:
        if rs["resource"] == "ip":
            for i in range(rs["count"]):
                instance_sequence = i

                ip_name = (
                    "pip"
                    + project_name
                    + "-"
                    + project_type
                    + "-"
                    + str(instance_sequence + 1)
                )
                resource = {
                    "provider": "azure",
                    "service_name": "Microsoft.Network",
                    "service_type": "IP Address",
                    "resource_name": ip_name,
                    "resource_id": f"/subscriptions/{subscription_id}/resourcegroups/{resource_group_name}/providers/microsoft.network/publicipaddresses/{ip_name}",
                    "region": region,
                    "pool": "Azure "
                    + project_name[0].upper()
                    + project_name[1:].lower()
                    + " "
                    + project_type.lower(),
                    "meta": {},
                }
                resources.append(resource)

        if rs["resource"] == "vm":
            for i in range(rs["count"]):
                instance_sequence = i

                ## Instance

                vpc_name = project_name + "vnet" + str(random.randint(100, 999))
                instance_name = (
                    project_name + "-" + project_type + "-" + str(instance_sequence + 1)
                )

                resource = {
                    "provider": "azure",
                    "service_name": "Microsoft.Compute",
                    "service_type": "Instance",
                    "resource_name": instance_name,
                    "resource_id": f"/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/microsoft.compute/virtualMachines/{instance_name}",
                    "region": region,
                    "pool": "Azure "
                    + project_name[0].upper()
                    + project_name[1:].lower()
                    + " "
                    + project_type.lower(),
                    "meta": {
                        "OS": "Linux",
                        "Size": "Standard_B2ms",
                        "VPC id": f"/subscriptions/{subscription_id}/resourcegroups/{resource_group_name}/providers/Microsoft.Network/virtualNetworks/{vpc_name}",
                        "VPC name": vpc_name,
                    },
                }
                resources.append(resource)

                ## Volume
                disk_name = "OsDisk_" + instance_name
                resource = {
                    "provider": "azure",
                    "service_name": "Microsoft.Compute",
                    "service_type": "Volume",
                    "resource_name": disk_name,
                    "resource_id": f"/subscriptions/{subscription_id}/resourcegroups/{resource_group_name}/providers/microsoft.compute/disks/{disk_name}",
                    "region": region,
                    "pool": "Azure "
                    + project_name[0].upper()
                    + project_name[1:].lower()
                    + " "
                    + project_type.lower(),
                    "meta": {
                        "Size": "2 TiB",
                        "Volume type": "Microsoft.Compute/disks",
                        "Attached": "True",
                    },
                }
                resources.append(resource)

                ## Snapshot
                snapshot_name = "snap_" + instance_name
                resource = {
                    "provider": "azure",
                    "service_name": "Microsoft.Compute",
                    "service_type": "Snapshot",
                    "resource_name": snapshot_name,
                    "resource_id": f"/subscriptions/{subscription_id}/resourcegroups/{resource_group_name}/providers/microsoft.compute/snapshots/{snapshot_name}",
                    "region": region,
                    "pool": "Azure "
                    + project_name[0].upper()
                    + project_name[1:].lower()
                    + " "
                    + project_type.lower(),
                    "meta": {
                        "Size": "10 GiB",
                        "State": "Succeeded",
                    },
                }
                resources.append(resource)


#    for r in resources:
#        print(r)


create_azure_resource(config)
