use std::path::PathBuf;

use clap::Parser;

/// Converts a Curriculum Vitae written in YAML files into LaTeX.
/// The format used in YAML files supports various curriculum item
/// types and allows providing alternate fields (e.g. for different
/// languages). You can create a dictionary with common terms and
/// use them throughout the YAML file structure. Any displayable
/// text inside YAML files supports Markdown for easy formatting.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The base YAML file to use as input
    pub input: PathBuf,

    /// The output LaTeX file
    pub latex: PathBuf,

    /// Whether to also produce an Europass XML file to attach to the PDF
    #[arg(short('x'), long)]
    pub attach_europass_xml: bool,

    /// Make GET requests to all links in the document and check
    /// if the response's status code is different from "200 OK"
    #[arg(short, long)]
    pub check_links: bool,
}