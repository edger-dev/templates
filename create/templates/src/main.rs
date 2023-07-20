use clap::Parser;
use edger_tui_app::prelude::{ConfigLoaderBuilder, TemplateWriterBuilder};

use {{ package_name }}::Args;

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
    {{ package_name }}::generate(&writer, &config);
}
