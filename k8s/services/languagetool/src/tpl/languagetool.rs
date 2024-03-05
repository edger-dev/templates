use askama::Template;
use deref_derive::Deref;

use crate::{Config, TemplateWriter};

#[derive(Template, Deref)]
#[template(path = "languagetool/deployment.yaml")]
pub struct Deployment(Config);

#[derive(Template, Deref)]
#[template(path = "languagetool/service.yaml")]
pub struct Service(Config);

#[derive(Template, Deref)]
#[template(path = "languagetool/ingress/traefik.yaml")]
pub struct IngressTraefik(Config);

pub fn generate(writer: &TemplateWriter, config: &Config) {
    let writer = writer.in_folder("languagetool");
    writer.write("deployment.yaml", &Deployment(config.clone()));
    writer.write("service.yaml", &Service(config.clone()));
    writer.write("ingress.yaml", &IngressTraefik(config.clone()));
}