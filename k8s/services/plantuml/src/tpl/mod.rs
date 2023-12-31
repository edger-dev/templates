pub mod root;
pub mod plantuml_server;

pub fn generate(writer: &crate::TemplateWriter, config: &crate::Config) {
    root::generate(writer, config);
    plantuml_server::generate(writer, config);
}
