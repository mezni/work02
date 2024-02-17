import pandas as pd
import numpy as np
import psycopg2
from psycopg2 import sql
from psycopg2.extensions import register_adapter, AsIs

# Define the start and end dates for your date dimension
start_date = "2023-01-01"
end_date = "2024-12-31"

# Generate a range of dates
dates = pd.date_range(start=start_date, end=end_date)


# Create a DataFrame to store the date dimension
date_dimension = pd.DataFrame(
    {
        "date_value": dates,
        "year": dates.year,
        "quarter": dates.quarter,
        "month": dates.month,
        "day": dates.day,
        "day_of_week": dates.dayofweek,  # Monday=0, Sunday=6
        "day_of_year": dates.dayofyear,
        "week_of_year": dates.isocalendar().week,
        "fiscal_year": dates.year,
        "fiscal_quarter": dates.quarter,
        "fiscal_month": dates.month,
        "is_weekend": dates.dayofweek.isin([5, 6]),  # Saturday or Sunday
        "is_holiday": dates.isin(
            ["2024-01-01", "2024-07-04", "2024-12-25"]
        ),  # Define your holidays here
        # Add more attributes as needed
    }
)

# Display the date dimension
# print(date_dimension)


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


# Define a function to adapt numpy types
def adapt_numpy_int64(numpy_int64):
    return AsIs(numpy_int64)


# Register numpy types for adaptation
register_adapter(np.int64, adapt_numpy_int64)


# Define the table name
table_name = "date_dimension"

# Create a list of column names
columns = date_dimension.columns.tolist()

# Create a placeholder for the VALUES string
values_placeholder = ", ".join(["%s" for _ in columns])

# Create the INSERT INTO statement
insert_query = sql.SQL("INSERT INTO {} ({}) VALUES ({})").format(
    sql.Identifier(table_name),
    sql.SQL(", ").join(map(sql.Identifier, columns)),
    sql.SQL(", ").join([sql.Placeholder()] * len(columns)),
)

# Convert DataFrame to list of tuples
data = [tuple(x) for x in date_dimension.to_numpy()]

# Execute the query to insert data into the PostgreSQL table
cur.executemany(insert_query, data)

# Commit the transaction
conn.commit()

# Close the cursor and connection
cur.close()
conn.close()
