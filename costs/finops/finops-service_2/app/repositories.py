import pandas as pd
import numpy as np


async def get_dim_date_all(payload: dict, session):
    pass


async def get_dim_date_max(session):
    record = await session.fetch("SELECT max(date_value) FROM date_dim")


async def create_dim_date(payload: dict, session):
    start_date = payload.start_date
    end_date = payload.end_date
    dates = pd.date_range(start=start_date, end=end_date)
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
            "is_holiday": dates.isin(["2024-01-01", "2024-07-04", "2024-12-25"]),
        }
    )

    table_name = "date_dim"

    columns = ",".join(date_dimension.columns)
    values_placeholder = ",".join(
        ["${}".format(i + 1) for i in range(len(date_dimension.columns))]
    )

    insert_query = f"INSERT INTO {table_name} ({columns}) VALUES ({values_placeholder})"

    async with session.transaction():
        for index, row in date_dimension.iterrows():
            await session.execute(insert_query, *row)

    return {"message": "OK"}
