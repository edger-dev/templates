use std::path::PathBuf;
use edger_tui_app::prelude::{VerboseArg, path_arg};

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[command(flatten)]
    pub verbose: VerboseArg,
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