use std::{io::Write, fmt::Debug};

use crate::printers::{rmarkdown::{RMarkdownPrinter, RMarkdownSectionItem}, printer::Printer};

#[derive(Debug)]
struct SectionElement<T> {
    title: String,
    description: Option<String>,
    items: Vec<T>,
}

impl<T: RMarkdownPrinter + RMarkdownSectionItem + Debug> RMarkdownPrinter
    for SectionElement<T>
{
    fn rmarkdown_print(&self, f: &mut Printer) -> std::io::Result<()> {
        writeln!(f, "# {}\n", self.title)?;

        if let Some(description) = &self.description {
            writeln!(f, "{description}")?;
        }

        writeln!(f, "```{{r section}}\ntribble(")?;

        let fields = T::get_field_names();
        writeln!(f, "  {}", fields.join(", "))?;

        for item in &self.items {
            write!(f, ",\n  ")?;
            item.rmarkdown_print(f)?;
        }
        Ok(())
    }
}
