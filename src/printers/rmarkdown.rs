use super::printer::Printer;
use std::{io::Write, fmt::Debug};

pub trait RMarkdownPrinter : Debug {
    fn rmarkdown_print(&self, f: &mut Printer) -> std::io::Result<()>;
}

pub trait RMarkdownSectionItem {
    fn get_field_names() -> &'static [String];
    fn get_fields(&self) -> Vec<String>;
}

impl<T: RMarkdownSectionItem + Debug> RMarkdownPrinter for T
{
    fn rmarkdown_print(&self, f: &mut Printer) -> std::io::Result<()> {
        let fields = self.get_fields();
        assert_eq!(Self::get_field_names().len(), fields.len());
        write!(f, "\"{}\"", fields[0])?;
        for field in &fields[1..] {
            write!(f, ", \"{field}\"")?;
        }
        Ok(())
    }
}
