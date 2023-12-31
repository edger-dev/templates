use askama::Template;
use deref_derive::Deref;

use crate::{Config, TemplateWriter};

#[derive(Template, Deref)]
#[template(path = "templates/justfile")]
pub struct Justfile(Config);

pub fn generate(writer: &TemplateWriter, config: &Config) {
    let writer = writer.in_folder("templates");
    writer.write("justfile", &Justfile(config.clone()));
}

