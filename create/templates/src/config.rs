#[derive(Debug, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
}

impl Default for Config {
    fn default() -> Self {
        Self {
        }
    }
}