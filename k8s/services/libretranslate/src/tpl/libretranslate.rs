use askama::Template;
use deref_derive::Deref;

use crate::{Config, TemplateWriter};

#[derive(Template, Deref)]
#[template(path = "libretranslate/deployment.yaml")]
pub struct Deployment(Config);

#[derive(Template, Deref)]
#[template(path = "libretranslate/service.yaml")]
pub struct Service(Config);

#[derive(Template, Deref)]
#[template(path = "libretranslate/ingress/traefik.yaml")]
pub struct IngressTraefik(Config);

pub fn generate(writer: &TemplateWriter, config: &Config) {
    let writer = writer.in_folder("libretranslate");
    writer.write("deployment.yaml", &Deployment(config.clone()));
    writer.write("service.yaml", &Service(config.clone()));
    writer.write("ingress.yaml", &IngressTraefik(config.clone()));
}