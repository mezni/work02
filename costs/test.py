import json
import pandas as pd

file_name = "data.json"

with open(file_name, "r") as file:
    data = json.load(file)
# print(data["costs"])
# for x in data["costs"]:
#    print(x)

df = pd.DataFrame(data["costs"])
print(df.head())

column_name = "Date"
distinct_values = df[column_name].unique()
print(distinct_values)

column_name = "Account"
distinct_values = df[column_name].unique()
print(distinct_values)

column_name = "Service"
distinct_values = df[column_name].unique()
print(distinct_values)
