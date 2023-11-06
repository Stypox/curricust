pub mod latex_writer;
pub mod markdown_to_latex;
pub mod write;

use std::fmt::Debug;

use self::latex_writer::LatexWriter;


pub trait AllWriters: LatexWriter + Debug {}

impl<T: LatexWriter + Debug> AllWriters for T {}
