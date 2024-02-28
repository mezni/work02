import asyncio
import asyncpg

import pandas as pd


class Processor:
    def __init__(self, db_config: dict) -> None:
        self.db_config = db_config

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

    async def process(self, df: pd.DataFrame):
        schema_colums = ["org"]
        for sc in schema_colums:
            print(df[sc].head())

    async def get_df(self, query: str) -> pd.DataFrame:
        pass


db_config = {
    "user": "finops",
    "password": "password123",
    "database": "finops-db",
    "host": "172.18.0.2",
}

data = {
    "date": ["2024-02-27", "2024-02-27", "2024-02-27"],
    "org": ["momentum", "momentum", "momentum_file"],
    "provider": ["aws", "aws", "aws"],
    "service": ["EC2", "S3", "Lambda"],
    "cost": [0.0001, 0.0002, 0.0003],
}
df = pd.DataFrame(data)


async def main():
    p = Processor(db_config)
    await p.process(df)


asyncio.run(main())
