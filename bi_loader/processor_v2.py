import asyncio
import asyncpg

import pandas as pd


class Processor:
    def __init__(self, db_config: dict) -> None:
        self.db_config = db_config
        self.connection = None

    async def connect(self):
        self.connection = await asyncpg.connect(
            user=self.db_config["user"],
            password=self.db_config["password"],
            database=self.db_config["database"],
            host=self.db_config["host"],
        )

    async def disconnect(self):
        if self.connection:
            await self.connection.close()

    async def get_df(self, query: str) -> pd.DataFrame:
        try:
            result = await self.connection.fetch(query)
            return pd.DataFrame(result, columns=result[0].keys())
        except:
            return pd.DataFrame()

    async def insert_df(self, df: pd.DataFrame, table_name: str) -> None:
        columns = ", ".join(df.columns)
        placeholders = ", ".join(f"${i+1}" for i in range(len(df.columns)))
        query = f"INSERT INTO {table_name} ({columns}) VALUES ({placeholders})"
        records = [tuple(row) for row in df.values]
        await self.connection.executemany(query, records)

    async def get_df_minus(self, df1, cols1, df2, cols2) -> pd.DataFrame:
        df_insert = pd.DataFrame()
        if df1.empty:
            df_insert = df2
        else:
            df_merged = pd.merge(
                df2,
                df1,
                left_on=cols2,
                right_on=cols1,
                how="outer",
                indicator=True,
            )
            df_diff = df_merged[df_merged["_merge"] == "left_only"]
            if not df_diff.empty:
                df_insert = df_diff[cols2]
                if not isinstance(df_insert, pd.DataFrame):
                    df_insert = df_insert.to_frame()
        return df_insert

    async def process(self, meta: dict, df: pd.DataFrame):
        await self.connect()
        org = meta.get("org", "")
        provider = meta.get("provider", "")
        table_name = "org_dim"
        schema_cols = ["org_name"]
        db_cols = ["org_name"]
        df_stored = await self.get_df(f"SELECT * FROM {table_name}")
        df_input = pd.DataFrame({"org_name": [org]}).drop_duplicates()
        df_insert = await self.get_df_minus(df_stored, db_cols, df_input, schema_cols)
        if not df_insert.empty:
            df_insert.columns = db_cols
            await self.insert_df(df_insert, table_name)

        table_name = "account_dim"
        schema_cols = ["account_name_xx", "account_id_xx"]
        db_cols = ["account_name", "account_id"]
        df_stored = await self.get_df(f"SELECT * FROM {table_name}")
        df_input = df[schema_cols].drop_duplicates()
        df_insert = await self.get_df_minus(df_stored, db_cols, df_input, schema_cols)
        if not df_insert.empty:
            df_insert.columns = db_cols
            await self.insert_df(df_insert, table_name)

        await self.disconnect()


db_config = {
    "user": "finops",
    "password": "passw0rd",
    "database": "finops",
    "host": "172.18.0.2",
}

dimensions = [
    {
        "table_name": "account_dim",
        "mapping": [
            {"schema_col": "account_name", "db_col": "account_name"},
            {"schema_col": "account_id", "db_col": "account_id"},
        ],
        "table_name": "service_dim",
        "mapping": [
            {"schema_col": "service_name", "db_col": "service_name"},
            {"schema_col": "service_id", "db_col": "service_id"},
        ],
    }
]

data = {
    "date": ["2024-02-27", "2024-02-27", "2024-02-27"],
    "account_name_xx": ["XX20240229", "XX20240228", "XX20240227"],
    "account_id_xx": ["20240229", "20240228", "20240227"],
    "service": ["EC2", "S3", "Lambda"],
    "cost": [0.0001, 0.0002, 0.0003],
}
df = pd.DataFrame(data)

meta = {"org": "momentum1", "provider": "aws"}

for d in dimensions:
    print(d)


async def main():
    p = Processor(db_config)
    await p.process(meta, df)


asyncio.run(main())
