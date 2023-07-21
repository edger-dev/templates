pub mod root;
pub mod src;

pub fn generate(writer: &crate::TemplateWriter, config: &crate::Config) {
    root::generate(writer, config);
    src::generate(writer, config);
}