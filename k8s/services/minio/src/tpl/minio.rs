use askama::Template;
use deref_derive::Deref;

use crate::{Config, TemplateWriter};

#[derive(Template, Deref)]
#[template(path = "minio/deployment.yaml")]
pub struct Deployment(Config);

#[derive(Template, Deref)]
#[template(path = "minio/service.yaml")]
pub struct Service(Config);

#[derive(Template, Deref)]
#[template(path = "minio/ingress/traefik/api.yaml")]
pub struct IngressTraefikApi(Config);

#[derive(Template, Deref)]
#[template(path = "minio/ingress/traefik/console.yaml")]
pub struct IngressTraefikConsole(Config);

pub fn generate(writer: &TemplateWriter, config: &Config) {
    let writer = writer.in_folder("minio");
    writer.write("deployment.yaml", &Deployment(config.clone()));
    writer.write("service.yaml", &Service(config.clone()));
    writer.write("ingress.api.yaml", &IngressTraefikApi(config.clone()));
    writer.write("ingress.console.yaml", &IngressTraefikConsole(config.clone()));
}