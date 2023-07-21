use askama::Template;
use deref_derive::Deref;

use crate::{Config, TemplateWriter};

#[derive(Template, Deref)]
#[template(path = "src/args.rs")]
pub struct SrcArgs(Config);

#[derive(Template, Deref)]
#[template(path = "src/config.rs")]
pub struct SrcConfig(Config);

#[derive(Template, Deref)]
#[template(path = "src/lib.rs")]
pub struct SrcLib(Config);

#[derive(Template, Deref)]
#[template(path = "src/main.rs")]
pub struct SrcMain(Config);


pub fn generate(writer: &TemplateWriter, config: &Config) {
    let writer = writer.in_folder("src");
    writer.write("args.rs", &SrcArgs(config.clone()));
    writer.write("config.rs", &SrcConfig(config.clone()));
    writer.write("lib.rs", &SrcLib(config.clone()));
    writer.write("main.rs", &SrcMain(config.clone()));
}