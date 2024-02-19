import psycopg2
import pandas as pd


class StoreHandler:
    def __init__(self, db_config) -> None:
        self.db_config = db_config
        self.conn = None
        self.cur = None

    def connect(self) -> None:
        self.conn = psycopg2.connect(**self.db_config)
        self.cur = self.conn.cursor()

    def disconnect(self) -> None:
        if self.cur:
            self.cur.close()
        if self.conn:
            self.conn.close()

    def select_to_df(self, table_name):
        self.connect()
        try:
            select_query = f"SELECT * FROM {table_name}"
            self.cur.execute(select_query)
            columns = [desc[0] for desc in self.cur.description]
            data = self.cur.fetchall()
            df = pd.DataFrame(data, columns=columns)
            return df
        except Exception as e:
            print(f"Error selecting data: {str(e)}")
            return pd.DataFrame()
        finally:
            self.disconnect()


db_config = {
    "user": "analytics",
    "password": "analytics",
    "host": "172.18.0.2",
    "port": "5432",
    "database": "analytics",
}

store_hdl = StoreHandler(db_config)
df_store_provider = store_hdl.select_to_df("provider_dimension")


list_provider = [(1, "aws"), (2, "azure"), (3, "oci")]
df_provider = pd.DataFrame(list_provider, columns=["key_id", "name"])

column_list = ["name"]
merged = pd.merge(
    df_provider, df_store_provider, on=column_list, how="left", indicator=True
)

# Filter the rows that are present in df1 but not in df2
result = merged[merged["_merge"] == "left_only"].drop(columns="_merge")
print(result["name"].head())
