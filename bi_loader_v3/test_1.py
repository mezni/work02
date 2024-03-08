import uuid
import random

project_name = "metalabs"
project_type = "dev"
resource_group_name = project_name + "-rg"
subscription_id = str(uuid.uuid4())
vpc_name = project_name + "vnet" + str(random.randint(100, 999))
vpc_id = f"/subscriptions/{subscription_id}/resourcegroups/{resource_group_name}/providers/Microsoft.Network/virtualNetworks/{vpc_name}"
instance_name = project_name + "-" + project_type + "-" + "1"
instance_id = f"/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/microsoft.compute/virtualMachines/{instance_name}"
resource = {
    "service": "Microsoft.Compute",
    "type": "Instance",
    "resource_name": instance_name,
    "resource_id": instance_id,
    "meta": {
        "OS": "Linux",
        "Size": "Standard_B2ms",
        "VPC id": vpc_id,
        "VPC name": vpc_name,
    },
}
print(resource)
# Microsoft.Compute	Instance	/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/microsoft.compute/virtualMachines/{instance_name}	OS=Linux|Size=Standard_B2ms|VPC id= |VPC name=
# Microsoft.Compute	Volume	/subscriptions/{subscription_id}/resourcegroups/{resource_group_name}/providers/microsoft.compute/disks/{disk_name}	Size=2 TiB|Volume type=Microsoft.Compute/disks|Attached=True	test_OsDisk_1_be1a5a03159348d78fe38f028a1ad90f
# Microsoft.Compute	Snapshot	/subscriptions/{subscription_id}/resourcegroups/{resource_group_name}/providers/microsoft.compute/snapshots/{snapshot_name}	Size=10 GiB|State=Succeeded	tm-aqa-westus2-underutilized-instance
# microsoft.network	IP Address	/subscriptions/{subscription_id}/resourcegroups/{resource_group_name}/providers/microsoft.network/publicipaddresses/{ip_name}
# Microsoft.Storage	Bucket	/subscriptions/{subscription_id}/resourcegroups/{resource_group_name}/providers/microsoft.storage/storageaccounts/publicipaddresses/{bucket_name}
