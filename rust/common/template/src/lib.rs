pub mod args;
pub mod config;

pub mod tpl;

pub use args::Args;
pub use config::Config;
pub use edger_tui_app::prelude::TemplateWriter;

pub fn generate(writer: &TemplateWriter, config: &Config) {
    tpl::root::generate(writer, config);
    tpl::src::generate(writer, config);
    tpl::tpl::generate(writer, config);
    tpl::templates::generate(writer, config);
}
