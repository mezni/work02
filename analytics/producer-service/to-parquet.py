import pandas as pd

df = pd.read_csv("events_20231109180000.csv")
df.to_parquet(
    "events_20231109180000.parquet", engine="fastparquet", compression="snappy"
)
