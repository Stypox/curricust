use super::printer::Printer;
use std::{io::Write, fmt::Debug};

pub trait RMarkdownPrinter : Debug {
    fn rmarkdown_print(&self, f: &mut Printer) -> std::io::Result<()>;
}

pub trait RMarkdownSectionItem {
    const N: usize;

    fn get_field_names() -> [&'static str; Self::N];
    fn get_fields(&self) -> [String; Self::N];
}

impl<T: RMarkdownSectionItem + Debug> RMarkdownPrinter for T
where
    [(); Self::N]:,
{
    fn rmarkdown_print(&self, f: &mut Printer) -> std::io::Result<()> {
        let fields = self.get_fields();
        if T::N != 0 {
            write!(f, "\"{}\"", fields[0])?;
            for field in &fields[1..] {
                write!(f, ", \"{field}\"")?;
            }
        }
        Ok(())
    }
}
