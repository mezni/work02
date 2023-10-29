from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    DB_NAME: str
    SUBSCRIBER_COUNT: int
    IP_COUNT: int

    class Config:
        env_file = "./.env"


settings = Settings()
