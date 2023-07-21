pub mod root;

pub fn generate(writer: &crate::TemplateWriter, config: &crate::Config) {
    root::generate(writer, config);
}