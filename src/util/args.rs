use std::path::PathBuf;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub input: PathBuf,
    pub output: PathBuf,

    #[arg(short, long)]
    pub check_links: bool,
}