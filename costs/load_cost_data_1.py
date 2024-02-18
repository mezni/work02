import pandas as pd
import numpy as np
import psycopg2
from psycopg2 import sql
from psycopg2.extensions import register_adapter, AsIs

df = pd.read_csv("finops.csv")

# Define your PostgreSQL connection parameters
db_config = {
    "user": "analytics",
    "password": "analytics",
    "host": "172.18.0.2",
    "port": "5432",
    "database": "analytics",
}

# Create a connection to the PostgreSQL database
conn = psycopg2.connect(**db_config)

cur = conn.cursor()


def insert(table_name, table_column_name, column_name):
    values_list = list(set(df[column_name].tolist()))

    sql_query = f"""
    INSERT INTO {table_name} ({table_column_name}) 
    VALUES (%s);
    """

    # Execute the SQL statement for each value in the list
    for value in values_list:
        cur.execute(sql_query, (value,))

    # Commit the transaction
    conn.commit()


# insert("client_dimension", "name", "Client")
# insert("provider_dimension", "name", "Provider")


# insert_data():
