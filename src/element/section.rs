use std::{fmt::Debug, io::Write};

use crate::printers::{
    printer::Printer,
    rmarkdown::{RMarkdownPrinter, RMarkdownSectionItem},
};

#[derive(Debug)]
pub struct SectionElement<T> {
    pub title: String,
    pub description: Option<String>,
    pub items: Vec<T>,
}

impl<T: RMarkdownPrinter + RMarkdownSectionItem> RMarkdownPrinter for SectionElement<T> {
    fn rmarkdown_print(&self, f: &mut Printer) -> std::io::Result<()> {
        writeln!(f, "# {}\n", self.title)?;

        if let Some(description) = &self.description {
            writeln!(f, "{description}\n")?;
        }

        writeln!(f, "```{{r section}}\ntribble(")?;

        let fields = T::get_field_names();
        write!(f, "  ~ {}", fields.join(", ~ "))?;

        for item in &self.items {
            write!(f, ",\n  ")?;
            item.rmarkdown_print(f)?;
        }
        writeln!(f, "\n)")?;
        writeln!(f, "```")?;
        Ok(())
    }
}
