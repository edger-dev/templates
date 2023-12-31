use askama::Template;
use deref_derive::Deref;

use crate::{Config, TemplateWriter};

#[derive(Template, Deref)]
#[template(path = "src/tpl/mod.rs")]
pub struct SrcTpl(Config);

#[derive(Template, Deref)]
#[template(path = "src/tpl/root.rs")]
pub struct SrcRoot(Config);

pub fn generate(writer: &TemplateWriter, config: &Config) {
    let writer = writer
        .in_folder("src")
        .in_folder("tpl");
    writer.write("mod.rs", &SrcTpl(config.clone()));
    writer.write("root.rs", &SrcRoot(config.clone()));
}
