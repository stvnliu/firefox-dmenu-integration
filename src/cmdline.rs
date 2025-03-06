use std::path::PathBuf;

use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to browser executable
    #[arg(short, long)]
    pub browser: PathBuf,
    /// Path to dmenu executable
    #[arg(short = 'm', long)]
    pub dmenu: PathBuf,
    /// Location history sqlite database
    #[arg(short = 'p', long, default_value = PathBuf::from("~/.mozilla/firefox/000000.default").into_os_string())]
    pub profile: PathBuf,
    /// Limit of location history entries to seek
    #[arg(short = 'l', long, default_value_t = 100)]
    pub limit: usize,
}
