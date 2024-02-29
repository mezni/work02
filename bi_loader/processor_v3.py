import asyncio
import asyncpg

import pandas as pd


class Processor:
    def __init__(self, db_config: dict) -> None:
        self.db_config = db_config
        self.connection = None

    async def connect(self) -> None:
        self.connection = await asyncpg.connect(
            user=self.db_config["user"],
            password=self.db_config["password"],
            database=self.db_config["database"],
            host=self.db_config["host"],
        )

    async def disconnect(self) -> None:
        if self.connection:
            await self.connection.close()

    async def get_df(self, query: str) -> pd.DataFrame:
        try:
            result = await self.connection.fetch(query)
            return pd.DataFrame(result, columns=result[0].keys())
        except:
            return pd.DataFrame()

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
        stmts = []
        await self.connect()
        org = meta.get("org", "")
        table_name = "org_dim"
        schema_cols = ["org_name"]
        db_cols = ["org_name"]
        df_stored = await self.get_df(f"SELECT * FROM {table_name}")
        df_input = pd.DataFrame({"org_name": [org]}).drop_duplicates()
        df_insert = await self.get_df_minus(df_stored, db_cols, df_input, schema_cols)
        columns = ", ".join(df_insert.columns)
        placeholders = ", ".join(f"${i+1}" for i in range(len(df_insert.columns)))
        query = f"INSERT INTO {table_name} ({columns}) VALUES ({placeholders})"
        records = [tuple(row) for row in df_insert.values]
        for row in df_insert.values:
            stmt = {"query": query, "values": tuple(row)}
            stmts.append(stmt)
        for stmt in stmts:
            await self.connection.execute(stmt["query"], *tuple(stmt["values"]))
        await self.disconnect()


db_config = {
    "user": "finops",
    "password": "passw0rd",
    "database": "finops",
    "host": "172.18.0.2",
}


data = {
    "date": ["2024-02-27", "2024-02-27", "2024-02-27"],
    "account_name_xx": ["XX20240229", "XX20240228", "XX20240227"],
    "account_id_xx": ["20240229", "20240228", "20240227"],
    "service": ["EC2", "S3", "Lambda"],
    "cost": [0.0001, 0.0002, 0.0003],
}
df = pd.DataFrame(data)

meta = {"org": "momentum1", "provider": "aws"}


async def main():
    p = Processor(db_config)
    await p.process(meta, df)


asyncio.run(main())
