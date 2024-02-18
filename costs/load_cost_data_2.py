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

    def insert_from_dataframe(self, df, table_name):
        self.connect()
        data = [tuple(row) for row in df.to_numpy()]
        columns = ", ".join(df.columns)
        placeholders = ", ".join(["%s" for _ in df.columns])
        insert_query = f"INSERT INTO {table_name} ({columns}) VALUES ({placeholders})"
        self.cur.executemany(insert_query, data)
        self.conn.commit()
        self.disconnect()

    def delete_from_dataframe(self, df, table_name):
        self.connect()
        data = [tuple(row) for row in df.to_numpy()]
        delete_query = f"DELETE FROM {table_name} WHERE id = %s"
        self.cur.executemany(delete_query, data)
        self.conn.commit()
        self.disconnect()

    def update_from_dataframe(self, df, table_name):
        self.connect()
        data = [tuple(row) for row in df.to_numpy()]
        update_query = (
            f"UPDATE {table_name} SET column1 = %s, column2 = %s WHERE id = %s"
        )
        self.cur.executemany(update_query, data)
        self.conn.commit()
        self.disconnect()

    def select_to_dataframe(self, table_name):
        self.connect()
        try:
            select_query = f"SELECT * FROM {table_name}"
            self.cur.execute(select_query)
            columns = [desc[0] for desc in self.cur.description]
            data = self.cur.fetchall()
            df = pd.DataFrame(data, columns=columns)
            print("Data selected successfully.")
            return df
        except Exception as e:
            print(f"Error selecting data: {str(e)}")
            return None
        finally:
            self.disconnect()
