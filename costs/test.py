import json

file_name = "data.json"

with open(file_name, "r") as file:
    data = json.load(file)
print(data)
