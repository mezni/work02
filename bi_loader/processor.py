import asyncio
import asyncpg
import pandas as pd


class Processor:
    def __init__(self, db_config: dict) -> None:
        self.db_config = db_config
        self.connection = None

    async def _connect(self) -> None:
        try:
            self.connection = await asyncpg.connect(
                user=self.db_config["user"],
                password=self.db_config["password"],
                database=self.db_config["database"],
                host=self.db_config["host"],
            )
        except Exception as e:
            raise ConnectionError(f"{e}")

    async def _disconnect(self) -> None:
        if self.connection is not None and not self.connection.is_closed():
            await self.connection.close()

    async def df_from_value(self, col, value) -> pd.DataFrame:
        df = pd.DataFrame({col: [value]}).drop_duplicates()
        return df

    async def get_df(self, query: str) -> pd.DataFrame:
        try:
            result = await self.connection.fetch(query)
            return pd.DataFrame(result, columns=result[0].keys())
        except:
            return pd.DataFrame()

    async def diff_df(self, df1, df2) -> pd.DataFrame:
        df_diff = pd.DataFrame()
        if not df2.empty:
            if df1.empty:
                df_diff = df2
            else:
                df1 = df1[df2.columns]
                df_merged = pd.merge(
                    df2,
                    df1,
                    how="outer",
                    indicator=True,
                )
                df_diff = df_merged[df_merged["_merge"] == "left_only"]
                df_diff = df_diff[df2.columns]
                if not df_diff.empty:
                    if not isinstance(df_diff, pd.DataFrame):
                        df_diff = df_diff.to_frame()
        return df_diff

    async def process(self, meta):
        try:
            p = await self._connect()

            for key, value in meta.items():
                if key == "org":
                    dim_conf = {
                        "table": "org_dim",
                        "db_cols": ["org_name"],
                        "sc_cols": ["org_name"],
                    }
                    df_input = await self.df_from_value(dim_conf["db_cols"][0], value)
                    df_stored = await self.get_df(f"SELECT * FROM {dim_conf['table']}")
                if key == "provider":
                    dim_conf = {
                        "table": "provider_dim",
                        "db_cols": ["provider_name"],
                        "sc_cols": ["provider_name"],
                    }
                    df_input = await self.df_from_value(dim_conf["db_cols"][0], value)
                    df_stored = await self.get_df(f"SELECT * FROM {dim_conf['table']}")
                df_insert = await self.diff_df(df_stored, df_input)

            return None
        except Exception as e:
            return f"{e}"
        finally:
            await self._disconnect()


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

meta = {"org": "momentum3", "provider": "aws"}


async def main():
    p = Processor(db_config)
    err = await p.process(meta)
    print(err)


asyncio.run(main())
