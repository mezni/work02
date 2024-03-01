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

    async def process(self):
        try:
            p = await self._connect()
            return None
        except Exception as e:
            return f"{e}"
        finally:
            await self._disconnect()


db_config = {
    "user": "finops",
    "password": "passw0rd",
    "database": "finops",
    "host": "172.18.0.3",
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
    err = await p.process()
    print(err)


asyncio.run(main())
