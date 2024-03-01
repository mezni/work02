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

        # WORK
        table_name = "account_dim"
        schema_cols = ["account_name_xx", "account_id_xx"]
        db_cols = ["account_name", "account_id"]
        df_stored = await self.get_df(f"SELECT * FROM {table_name}")
        df_input = df[schema_cols].drop_duplicates()

        if df_stored.empty:
            df_insert = df_input
        else:
            df_merged = pd.merge(
                df_input,
                df_stored,
                left_on=schema_cols,
                right_on=db_cols,
                how="outer",
                indicator=True,
            )
            df_diff = df_merged[df_merged["_merge"] == "left_only"]
            if not df_diff.empty:
                df_insert = df_diff[schema_cols]
                if not isinstance(df_insert, pd.DataFrame):
                    df_insert = df_insert.to_frame()

        if not df_insert.empty:
            df_insert.columns = db_cols
            await self.insert_df(df_insert, table_name)

        await self.disconnect()

    async def get_df_insert(
        self, df_stored, df_input, schema_cols, db_cols
    ) -> pd.DataFrame:
        df_insert = pd.DataFrame()
        if df_stored.empty:
            df_insert = df_input
        else:
            df_merged = pd.merge(
                df_input,
                df_stored,
                left_on=schema_cols,
                right_on=db_cols,
                how="outer",
                indicator=True,
            )
            df_diff = df_merged[df_merged["_merge"] == "left_only"]
            if not df_diff.empty:
                df_insert = df_diff[schema_cols]
                if not isinstance(df_insert, pd.DataFrame):
                    df_insert = df_insert.to_frame()
        return db_insert


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
    "account_name_xx": ["XX20240229", "XX20240228", "XX20240227"],
    "account_id_xx": ["20240229", "20240228", "20240227"],
    "service": ["EC2", "S3", "Lambda"],
    "cost": [0.0001, 0.0002, 0.0003],
}
df = pd.DataFrame(data)

meta = {"org": "momentum", "provider": "aws"}


async def main():
    p = Processor(db_config)
    await p.process(meta, df)


asyncio.run(main())
