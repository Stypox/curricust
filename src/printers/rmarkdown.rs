use std::io::Write;

pub trait RMarkdownPrinter<T> where T: Write {
    fn rmarkdown_print(&self, f: &mut T) -> std::io::Result<()>;
}