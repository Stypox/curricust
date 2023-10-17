use crate::printers::rmarkdown::{RMarkdownPrinter, RMarkdownSectionItem};

struct SectionElement<T> {
    title: String,
    description: Option<String>,
    items: Vec<T>,
}

// impl<T: RMarkdownPrinter + RMarkdownSectionItem> RMarkdownPrinter
//     for SectionElement<T>
// where
//     [(); T::N]:,
// {
//     fn rmarkdown_print<W: std::io::Write>(&self, f: &mut W) -> std::io::Result<()> {
//         writeln!(f, "# {}\n", self.title)?;

//         if let Some(description) = &self.description {
//             writeln!(f, "{description}")?;
//         }

//         writeln!(f, "```{{r section}}\ntribble(")?;

//         let fields = T::get_field_names();
//         writeln!(f, "  {}", fields.join(", "))?;

//         for item in &self.items {
//             write!(f, ",\n  ")?;
//             item.rmarkdown_print(f)?;
//         }
//         Ok(())
//     }
// }
