import asyncpg
import asyncio
import pandas as pd


class Processor:
    def __init__(self, df: pd.DataFrame) -> None:
        self.source_df = df

    async def print(self) -> None:
        print(self.source_df.head())

    async def get_df(self, query: str) -> pd.DataFrame:
        data = {"orga": ["", "momentum", "momentum_stored"]}
        return pd.DataFrame(data)

    async def process(self) -> None:
        df = self.source_df["org"]
        df_db = await self.get_df(query="")
        df_merged = pd.merge(
            df, df_db, left_on="org", right_on="orga", how="outer", indicator=True
        )
        df_diff = df_merged[df_merged["_merge"] == "left_only"]
        print("to insert")
        print(df_diff["org"].head())


data = {
    "date": ["2024-02-27", "2024-02-27", "2024-02-27"],
    "org": ["momentum", "momentum", "momentum_file"],
    "provider": ["aws", "aws", "aws"],
    "service": ["EC2", "S3", "Lambda"],
    "cost": [0.0001, 0.0002, 0.0003],
}
df = pd.DataFrame(data)


async def main():
    p = Processor(df)
    await p.process()


asyncio.run(main())
