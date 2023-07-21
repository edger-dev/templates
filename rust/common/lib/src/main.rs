use clap::Parser;
use edger_tui_app::prelude::{ConfigLoaderBuilder, TemplateWriterBuilder};

use rust_lib::Args;

fn main() {
    let args = Args::parse();
    edger_tui_app::prelude::init_tracing(&args.verbose);

    let config = ConfigLoaderBuilder::default()
        .toml_path(args.config_path())
        .build().unwrap()
        .load();
    let writer = TemplateWriterBuilder::default()
        .path(args.output_path())
        .build().unwrap();
    rust_lib::tpl::generate(&writer, &config);
}
