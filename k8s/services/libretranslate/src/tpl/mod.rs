pub mod root;
pub mod libretranslate;

pub fn generate(writer: &crate::TemplateWriter, config: &crate::Config) {
    root::generate(writer, config);
    libretranslate::generate(writer, config);
}