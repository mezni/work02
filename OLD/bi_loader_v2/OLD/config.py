import os


class Config:
    DB_CONFIG = os.getenv(
        "DB_CONFIG",
        "postgresql+asyncpg://{DB_USER}:{DB_PASSWORD}@{DB_HOST}/{DB_NAME}".format(
            DB_USER=os.getenv("DB_USER", "expenses"),
            DB_PASSWORD=os.getenv("DB_PASSWORD", "passw0rd"),
            DB_HOST=os.getenv("DB_HOST", "172.18.0.2:5432"),
            DB_NAME=os.getenv("DB_NAME", "expenses"),
        ),
    )


config = Config
