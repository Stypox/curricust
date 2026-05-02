use std::fs::File;

use clap::Parser;
use util::args::Args;

use crate::{
    element::base::BaseElement, util::{error::ErrorToString, file::yaml_from_file}, writer::{europass_xml_writer::ATTACHMENT_XML_FILENAME, write::MyWrite}
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

    let mut my_write = MyWrite::file(File::create(&args.latex).err_str()?);

    // write the element in latex using the MyWrite instance, which also
    // collects urls referenced in the document
    base_element
        .latex_write(&mut my_write, args.attach_europass_xml)
        .err_str()?;

    if args.attach_europass_xml {
        let xml = base_element.to_europass_xml().err_str()?;
        let mut xml_writer = File::create(
            args.latex.parent().ok_or("Cannot find parent folder")?.join(ATTACHMENT_XML_FILENAME)
        ).err_str()?;
        xml.generate(&mut xml_writer).unwrap();
    }

    if args.check_links {
        if let Err(url_errors) = my_write.check_urls().await {
            print!("{url_errors}")
        }
    }

    Ok(())
}
