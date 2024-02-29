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

    async def process(self, meta: dict, df: pd.DataFrame):
        org = meta.get("org", "")
        provider = meta.get("provider", "")
        await self.connect()
        df_stored = await self.get_df(f"SELECT * FROM org_dim")
        df_input = pd.DataFrame({"org_name": [org]})
        df_insert = pd.DataFrame()
        if df_stored.empty:
            df_insert = df_input
        else:
            df_merged = pd.merge(
                df_input,
                df_stored,
                left_on="org_name",
                right_on="org_name",
                how="outer",
                indicator=True,
            )
            df_diff = df_merged[df_merged["_merge"] == "left_only"]
            if not df_diff.empty:
                df_insert = df_diff["org_name"].to_frame()

        if not df_insert.empty:
            await self.insert_df(df_insert, "org_dim")

        df_stored = await self.get_df(f"SELECT * FROM provider_dim")
        df_input = pd.DataFrame({"provider_name": [provider]})
        df_insert = pd.DataFrame()
        if df_stored.empty:
            df_insert = df_input
        else:
            df_merged = pd.merge(
                df_input,
                df_stored,
                left_on="provider_name",
                right_on="provider_name",
                how="outer",
                indicator=True,
            )
            df_diff = df_merged[df_merged["_merge"] == "left_only"]
            if not df_diff.empty:
                df_insert = df_diff["provider_name"].to_frame()

        if not df_insert.empty:
            await self.insert_df(df_insert, "provider_dim")

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
    }
]

data = {
    "date": ["2024-02-27", "2024-02-27", "2024-02-27"],
    "account_name": ["20240229", "20240229", "20240229"],
    "account_id": ["20240229", "20240229", "20240229"],
    "service": ["EC2", "S3", "Lambda"],
    "cost": [0.0001, 0.0002, 0.0003],
}
df = pd.DataFrame(data)

meta = {"org": "momentum", "provider": "aws"}


async def main():
    p = Processor(db_config)
    await p.process(meta, df)


asyncio.run(main())
