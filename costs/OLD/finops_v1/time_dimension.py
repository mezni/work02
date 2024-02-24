import pandas as pd


class TimeDimension:
    def __init__(self) -> None:
        self.df = pd.DataFrame()

    def generate_date_range(self):
        pass

    def populate_dimension(self):
        pass

    def insert_into_db(self):
        pass

    def get_db_conn(self):
        pass

    def create_table(self):
        pass


time_dim = TimeDimension()
time_dim.populate_dimension()
time_dim.insert_into_db()
