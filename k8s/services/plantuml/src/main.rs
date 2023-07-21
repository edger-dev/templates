use clap::Parser;
use edger_tui_app::prelude::{ConfigLoaderBuilder, TemplateWriterBuilder};

use k8s_plantuml::Args;

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
    k8s_plantuml::tpl::generate(&writer, &config);
}