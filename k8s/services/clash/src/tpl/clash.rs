use askama::Template;
use deref_derive::Deref;

use crate::{Config, TemplateWriter};

#[derive(Template, Deref)]
#[template(path = "clash/deployment.yaml")]
pub struct Deployment(Config);

#[derive(Template, Deref)]
#[template(path = "clash/service.yaml")]
pub struct Service(Config);

#[derive(Template, Deref)]
#[template(path = "clash/config/ports.yaml")]
pub struct ConfigPorts(Config);

pub fn generate(writer: &TemplateWriter, config: &Config) {
    let writer = writer.in_folder("clash");
    writer.write("deployment.yaml", &Deployment(config.clone()));
    writer.write("service.yaml", &Service(config.clone()));
    writer.write_text("config.yaml",
        format!(
            "{}\n{}",
            ConfigPorts(config.clone()).render().unwrap(),
            config.extra_config,
        ));
}