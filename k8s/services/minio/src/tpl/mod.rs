pub mod root;
pub mod minio;

pub fn generate(writer: &crate::TemplateWriter, config: &crate::Config) {
    root::generate(writer, config);
    minio::generate(writer, config);
}