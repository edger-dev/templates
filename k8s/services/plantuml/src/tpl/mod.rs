pub mod just;
pub mod plantuml_server;

pub fn generate(writer: &crate::TemplateWriter, config: &crate::Config) {
    just::generate(writer, config);
    plantuml_server::generate(writer, config);
}