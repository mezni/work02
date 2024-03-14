import pandas as pd
import numpy as np
from datetime import datetime


async def create_holiday(payload: dict, session):
    table_name = "holidays"
    columns = ["date_value", "description"]
    values = (datetime.strptime(payload.holiday_date, "%Y-%m-%d"), payload.holiday_desc)
    query_columns = ",".join(columns)
    query_values = ",".join(["${}".format(i + 1) for i in range(len(columns))])
    insert_query = f"INSERT INTO {table_name} ({query_columns}) VALUES ({query_values})"
    await session.execute(insert_query, *values)
    return {"res": "OK"}
