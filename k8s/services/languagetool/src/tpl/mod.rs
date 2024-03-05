pub mod root;
pub mod languagetool;

pub fn generate(writer: &crate::TemplateWriter, config: &crate::Config) {
    root::generate(writer, config);
    languagetool::generate(writer, config);
}