#[derive(Debug, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub namespace: String,

    pub socks5_port: u16,
    pub socks5_node_port: u16,

    pub http_port: u16,
    pub http_node_port: u16,

    pub control_port: u16,
    pub control_node_port: u16,

    pub yacd_host: String,
    pub extra_config: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            namespace: "clash".to_string(),
            socks5_port: 1101,
            socks5_node_port: 31101,
            http_port: 1102,
            http_node_port: 31102,
            control_port: 1109,
            control_node_port: 31109,
            yacd_host: "yacd.localhost".to_string(),
            extra_config: "".to_string(),
        }
    }
}