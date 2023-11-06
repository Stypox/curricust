use std::{env, fs::File, path::PathBuf};

use crate::{
    element::base::BaseElement,
    writer::{latex_writer::LatexWriter, MyWrite},
    util::{error::ErrorToString, file::yaml_from_file},
};

extern crate yaml_rust;

mod attr;
mod element;
mod item;
mod writer;
mod util;
pub mod header;

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
    // println!("Yaml: {yaml:?}\n");
    let base_element = BaseElement::new(root, yaml)?;
    // println!("Base element: {base_element:?}\n");

    let outputfile = args.get(2).ok_or_else(usage)?;
    let outputfile = File::create(outputfile).err_str()?;
    base_element
        .latex_write(&mut MyWrite::File(outputfile))
        .err_str()?;
    Ok(())
}
