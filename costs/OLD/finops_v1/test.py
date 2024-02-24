table = {
    "table_name": "date_dim",
    "columns": [
        {"column_name": "date_key", "type": "SERIAL", "PRIMARY KEY": True},
        {"column_name": "date_full", "type": "DAY"},
        {"column_name": "day_of_week", "type": "INT"},
    ],
}

table_name = table["table_name"]
query = f"CREATE TABLE IF NOT EXISTS {table_name} ("
for col in table["columns"]:
    print(col["column_name"])
    column_name = col["column_name"]
    column_type = col["type"]
    pk = col.get("PRIMARY KEY", "")
    if pk:
        query += f"{column_name} {column_type} PRIMARY KEY, "
    else:
        query += f"{column_name} {column_type}, "
query = query.rstrip(", ")
query += ")"
print(query)
