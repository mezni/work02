import asyncpg
import pandas as pd


class PostgresDataFrame:
    def __init__(self, user, password, database, host):
        self.user = user
        self.password = password
        self.database = database
        self.host = host

    async def connect(self):
        self.connection = await asyncpg.connect(
            user=self.user,
            password=self.password,
            database=self.database,
            host=self.host,
        )

    async def insert_dataframe(self, table_name, df):
        await self.connect()
        try:
            await self.connection.copy_records_to_table(
                table_name, records=df.values.tolist()
            )
        finally:
            await self.disconnect()

    async def select_dataframe(self, query):
        await self.connect()
        try:
            result = await self.connection.fetch(query)
            return pd.DataFrame(result, columns=result[0].keys())
        finally:
            await self.disconnect()

    async def update_dataframe(
        self, table_name, condition_column, condition_value, update_column, update_value
    ):
        await self.connect()
        try:
            await self.connection.execute(
                f"UPDATE {table_name} SET {update_column} = $1 WHERE {condition_column} = $2",
                update_value,
                condition_value,
            )
        finally:
            await self.disconnect()

    async def delete_rows(self, table_name, condition_column, condition_value):
        await self.connect()
        try:
            await self.connection.execute(
                f"DELETE FROM {table_name} WHERE {condition_column} = $1",
                condition_value,
            )
        finally:
            await self.disconnect()

    async def disconnect(self):
        await self.connection.close()


# Example usage:
async def main():
    pgdf = PostgresDataFrame(
        user="your_username",
        password="your_password",
        database="your_database",
        host="localhost",
    )

    # Create a DataFrame
    df = pd.DataFrame(
        {"id": [1, 2, 3], "name": ["Alice", "Bob", "Charlie"], "age": [30, 25, 35]}
    )

    # Insert DataFrame into PostgreSQL table
    await pgdf.insert_dataframe("your_table_name", df)

    # Select data from PostgreSQL table into DataFrame
    result_df = await pgdf.select_dataframe("SELECT * FROM your_table_name")
    print("Selected data:")
    print(result_df)

    # Update data in PostgreSQL table
    await pgdf.update_dataframe("your_table_name", "name", "Alice", "age", 31)

    # Delete rows from PostgreSQL table
    await pgdf.delete_rows("your_table_name", "name", "Charlie")

    # Disconnect from the database
    await pgdf.disconnect()


# Run the main function
asyncio.run(main())
