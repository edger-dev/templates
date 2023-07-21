use clap::Parser;
use edger_tui_app::prelude::{ConfigLoaderBuilder, TemplateWriterBuilder};

use rust_template::Args;

fn main() {
    let args = Args::parse();
    edger_tui_app::prelude::init_tracing(&args.verbose);

    let config = ConfigLoaderBuilder::default()
        .toml_path(args.config_path())
        .build().unwrap()
        .load_with_args(&args);
    let writer = TemplateWriterBuilder::default()
        .path(args.output_path())
        .build().unwrap();
    rust_template::generate(&writer, &config);
}
