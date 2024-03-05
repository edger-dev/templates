use askama::Template;
use deref_derive::Deref;

use crate::{Config, TemplateWriter};

#[derive(Template, Deref)]
#[template(path = "justfile")]
pub struct Justfile(Config);

pub fn generate(writer: &TemplateWriter, config: &Config) {
    writer.write("justfile", &Justfile(config.clone()));
}