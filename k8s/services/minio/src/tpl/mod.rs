pub mod just;
pub mod minio;

pub fn generate(writer: &crate::TemplateWriter, config: &crate::Config) {
    just::generate(writer, config);
    minio::generate(writer, config);
}