#[derive(Debug, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub namespace: String,
    pub server_host: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            namespace: "libretranslate".into(),
            server_host: "libretranslate.localhost".into(),
        }
    }
}