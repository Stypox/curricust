//! https://www.overleaf.com/latex/templates/cv-developer/rdycxzvvnvcc

use super::Printer;
use std::io::Write;

pub trait CvDeveloperLatexPrinter {
    fn cvdl_print(&self, f: &mut Printer) -> std::io::Result<()>;
}

pub trait CvDeveloperLatexSectionItem {
    fn cvdl_print_left(&self, f: &mut Printer) -> std::io::Result<()>;
    fn cvdl_print_heading(&self, f: &mut Printer) -> std::io::Result<()>;
    fn cvdl_print_qualifier(&self, f: &mut Printer) -> std::io::Result<()>;
    fn cvdl_print_description(&self, f: &mut Printer) -> std::io::Result<()>;
}

impl<T: CvDeveloperLatexSectionItem> CvDeveloperLatexPrinter for T {
    fn cvdl_print(&self, f: &mut Printer) -> std::io::Result<()> {
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
