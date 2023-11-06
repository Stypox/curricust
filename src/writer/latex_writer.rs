//! https://www.overleaf.com/latex/templates/cv-developer/rdycxzvvnvcc

use super::{markdown_to_latex, Writer};
use std::io::Write;

pub trait LatexWriter {
    fn latex_write(&self, f: &mut Writer) -> std::io::Result<()>;
}

pub trait SectionItemLatexWriter: LatexWriter {
    const SECTION_COMMAND: &'static str;
}

pub fn write_latex_command_call(
    f: &mut Writer,
    command: &str,
    arguments: &[&str],
) -> std::io::Result<()> {
    write!(f, "\\")?;
    writeln!(f, "{command}")?;
    for argument in arguments {
        write!(f, "    {{")?;
        markdown_to_latex::write_markdown(f, argument)?;
        writeln!(f, "}}")?;
    }
    Ok(())
}
