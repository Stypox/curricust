pub mod latex_writer;
pub mod markdown_to_latex;
pub mod write;
pub mod europass_xml_writer;
pub mod markdown_utils;

use std::fmt::Debug;

use crate::writer::europass_xml_writer::EuropassXmlWriter;

use self::latex_writer::LatexWriter;


pub trait AllWriters: LatexWriter + EuropassXmlWriter + Debug {}

impl<T: LatexWriter + EuropassXmlWriter + Debug> AllWriters for T {}
