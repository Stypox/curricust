use std::fs::File;

use clap::Parser;
use util::args::Args;

use crate::{
    element::base::BaseElement,
    writer::{latex_writer::LatexWriter, write::MyWrite},
    util::{error::ErrorToString, file::yaml_from_file},
};

extern crate yaml_rust;

mod attr;
mod element;
mod item;
mod writer;
mod util;
mod header;

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();

    let root = args.input
        .parent()
        .ok_or("Input file does not have a parent")?;

    let yaml = yaml_from_file(&args.input)?;
    // println!("Yaml: {yaml:?}\n");
    let base_element = BaseElement::new(root, yaml)?;
    // println!("Base element: {base_element:?}\n");

    let mut my_write = MyWrite::file(File::create(args.output).err_str()?);

    // write the element in latex using the MyWrite instance, which also
    // collects urls referenced in the document
    base_element
        .latex_write(&mut my_write)
        .err_str()?;

    if args.check_links {
        if let Err(url_errors) = my_write.check_urls().await {
            print!("{url_errors}")
        }
    }

    Ok(())
}
