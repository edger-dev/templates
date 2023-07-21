use askama::Template;
use deref_derive::Deref;

use crate::{Config, TemplateWriter};

#[derive(Template, Deref)]
#[template(path = "src/lib.rs")]
pub struct SrcLib(Config);

#[derive(Template, Deref)]
#[template(path = "src/todo.rs")]
pub struct SrcTodo(Config);


pub fn generate(writer: &TemplateWriter, config: &Config) {
    let writer = writer.in_folder("src");
    writer.write("lib.rs", &SrcLib(config.clone()));
    writer.write("todo.rs", &SrcTodo(config.clone()));
}