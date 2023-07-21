use std::path::PathBuf;
use edger_tui_app::prelude::{VerboseArg, path_arg};

#[derive(Debug, clap::Parser, serde::Serialize)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    #[serde(skip_serializing)]
    pub config: Option<PathBuf>,

    #[arg(short, long)]
    #[serde(skip_serializing)]
    pub output: Option<PathBuf>,

    #[command(flatten)]
    #[serde(skip_serializing)]
    pub verbose: VerboseArg,

    #[arg(short, long)]
    pub package_name: String,
}

impl Args {
    pub const CONFIG_FILENAME: &'static str = "config.toml";
    pub const DEFAULT_OUTPUT: &'static str = "output";

    pub fn config_path(&self) -> PathBuf {
        path_arg::unwrap_or_in_cwd(&self.config, &vec![Self::CONFIG_FILENAME])
    }

    pub fn output_path(&self) -> PathBuf {
        path_arg::unwrap_or_in_cwd(&self.output, &vec![Self::DEFAULT_OUTPUT])
    }
}