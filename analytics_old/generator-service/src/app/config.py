from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    DB_NAME: str
    SUBSCRIBER_COUNT: int
    IP_COUNT: int

    model_config = SettingsConfigDict(env_file="./.env")


settings = Settings()
