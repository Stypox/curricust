//! https://www.overleaf.com/latex/templates/cv-developer/rdycxzvvnvcc

use super::Writer;
use std::io::Write;

pub trait LatexPrinter {
    fn cvdl_print(&self, f: &mut Writer) -> std::io::Result<()>;
}

pub trait LatexSectionItem {
    fn cvdl_print_left(&self, f: &mut Writer) -> std::io::Result<()>;
    fn cvdl_print_heading(&self, f: &mut Writer) -> std::io::Result<()>;
    fn cvdl_print_qualifier(&self, f: &mut Writer) -> std::io::Result<()>;
    fn cvdl_print_description(&self, f: &mut Writer) -> std::io::Result<()>;
}

impl<T: LatexSectionItem> LatexPrinter for T {
    fn cvdl_print(&self, f: &mut Writer) -> std::io::Result<()> {
        write!(f, "\\entry\n    {{")?;
        self.cvdl_print_left(f)?;
        write!(f, "}}\n    {{")?;
        self.cvdl_print_heading(f)?;
        write!(f, "}}\n    {{")?;
        self.cvdl_print_qualifier(f)?;
        write!(f, "}}\n    {{")?;
        self.cvdl_print_description(f)?;
        writeln!(f, "}}")?;
        Ok(())
    }
}
