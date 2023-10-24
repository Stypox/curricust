#![feature(variant_count)]

use std::{env, fs::File, path::PathBuf};

use crate::{
    element::base::BaseElement,
    printers::{cv_developer_latex_printer::CvDeveloperLatexPrinter, Printer},
    util::{error::ErrorToString, file::yaml_from_file},
};

extern crate yaml_rust;

mod attr;
mod element;
mod item;
mod printers;
mod util;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let usage = || {
        format!(
            "Usage: {} inputfile.yml outputfile.tex",
            args.get(0).unwrap_or(&"resume-cv-rust".to_string())
        )
    };
    if args.len() != 3 {
        return Err(usage());
    }

    let inputfile = args.get(1).ok_or_else(usage)?;
    let inputfile = PathBuf::from(inputfile);
    let root = inputfile
        .parent()
        .ok_or("Input file does not have a parent")?;

    let yaml = yaml_from_file(&inputfile)?;
    println!("{yaml:?}\n");
    let base_element = BaseElement::new(root, yaml)?;
    println!("Base element: {base_element:?}\n");

    let outputfile = args.get(2).ok_or_else(usage)?;
    let outputfile = File::create(outputfile).err_str()?;
    base_element
        .cvdl_print(&mut Printer::File(outputfile))
        .err_str()?;
    Ok(())
}
