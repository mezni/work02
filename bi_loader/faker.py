import pandas as pd
import random
from uuid_utils import uuid7


df_owners = pd.read_csv("data/owners.csv")
# print(df_owners.head())
df_resources = pd.read_csv("data/resources.csv")
# print(df_resources.head())
owners_list = df_owners.to_dict("records")
resources_list = df_resources.to_dict("records")

file_id = str(uuid7())
dates = [
    "2024-03-04",
    "2024-03-05",
    "2024-03-06",
]  # , ["2024-03-01","2024-03-02", "2024-03-03", "2024-03-04", "2024-03-05"]
owner_ids = [1, 2]

expenses = []
for owner_id in owner_ids:
    owner = [o for o in owners_list if o["owner_id"] == owner_id][0]
    for date in dates:
        for r in resources_list:
            if r["owner_id"] == owner_id:
                expense = {
                    "file_id": file_id,
                    "date": date,
                    "organization_name": owner["organization_name"],
                    "owner": owner["owner"],
                    "pool": owner["pool"],
                    "provider": owner["provider"],
                    "service_name": r["service_name"],
                    "resource_name": r["resource_name"],
                    "resource_id": r["resource_id"],
                    "resource_type": r["resource_type"],
                    "region": r["region"],
                    "metadata": r["metadata"],
                    "tags": r["tags"],
                    "cost_usd": round(random.random(), 6) * random.randint(0, 9),
                    "cost_type": "real",
                    "inserted_at": date + " 00:00:00",
                }
                expenses.append(expense)
df_expenses = pd.DataFrame(expenses)
df_expenses.to_csv("output/expenses_20240306.csv", index=False)
