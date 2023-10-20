#![feature(variant_count)]

use std::{env, path::PathBuf};

use crate::{
    element::base::BaseElement,
    printers::{printer::Printer, rmarkdown::RMarkdownPrinter},
    util::{error::ErrorToString, file::yaml_from_file},
};

extern crate yaml_rust;

mod element;
pub mod printers;
mod util;
pub mod attr;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let usage = || {
        format!(
            "Usage: {} filename.yml",
            args.get(0).unwrap_or(&"resume-cv-rust".to_string())
        )
    };
    if args.len() != 2 {
        return Err(usage());
    }
    let filename = args.get(1).ok_or_else(usage)?;
    let filename = PathBuf::from(filename);
    let root = filename.parent().ok_or("Filename does not have a parent")?;

    let yaml = yaml_from_file(&filename)?;
    println!("{yaml:?}\n");
    let base_element = BaseElement::new(root, yaml)?;
    println!("Base element: {base_element:?}\n");

    base_element.rmarkdown_print(&mut Printer {}).err_str()?;
    Ok(())
}
