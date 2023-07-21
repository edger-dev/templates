use askama::Template;
use deref_derive::Deref;

use crate::{Config, TemplateWriter};

#[derive(Template, Deref)]
#[template(path = "Cargo.toml")]
pub struct Cargo(Config);

#[derive(Template, Deref)]
#[template(path = "askama.toml")]
pub struct Askama(Config);

#[derive(Template, Deref)]
#[template(path = "gitignore")]
pub struct GitIgnore(Config);


pub fn generate(writer: &TemplateWriter, config: &Config) {
    writer.write("Cargo.toml", &Cargo(config.clone()));
    writer.write("askama.toml", &Askama(config.clone()));
    writer.write(".gitignore", &GitIgnore(config.clone()));
}

