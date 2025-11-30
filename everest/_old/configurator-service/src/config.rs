use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Settings {
    pub fn from_env() -> anyhow::Result<Self> {
        let s = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        s.try_deserialize::<Settings>().map_err(Into::into)
    }
}
