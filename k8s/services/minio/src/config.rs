#[derive(Debug, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub namespace: String,
    pub api_host: String,
    pub console_host: String,
    pub volume_size: String,
    pub with_https: bool,
    pub cert_manager_cluster_issuer: String,
    pub traefik_router_middlewares: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            namespace: "minio".into(),
            api_host: "minio.localhost".into(),
            console_host: "minio-ui.localhost".into(),
            volume_size: "10Gi".into(),
            with_https: false,
            cert_manager_cluster_issuer: "".into(),
            traefik_router_middlewares: "".into()
        }
    }
}