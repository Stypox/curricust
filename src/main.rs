use std::env;

use crate::{element::base::BaseElement, util::yaml::yaml_from_file};

extern crate yaml_rust;

mod element;
mod util;


fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let usage = || { format!("Usage: {} filename.yml", args.get(0).unwrap_or(&"resume-cv-rust".to_string())) };
    if args.len() != 2 {
        return Err(usage());
    }
    let filename = args.get(1).ok_or_else(usage)?;

    let yaml = yaml_from_file(filename)?;
    println!("{yaml:?}\n");
    let base_element = BaseElement::new(yaml)?;
    println!("Base element: {base_element:?}");
    Ok(())
}
