pub mod cv_developer_latex_printer;
pub mod rmarkdown;

use std::fs::File;

use self::{cv_developer_latex_printer::CvDeveloperLatexPrinter, rmarkdown::RMarkdownPrinter};

// keep Stdout and Stderr for easy testing
#[allow(dead_code)]
pub enum Printer {
    Stdout,
    Stderr,
    File(File),
}

impl std::io::Write for Printer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Printer::Stdout => std::io::stdout().write(buf),
            Printer::Stderr => std::io::stderr().write(buf),
            Printer::File(f) => f.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Printer::Stdout => std::io::stdout().flush(),
            Printer::Stderr => std::io::stderr().flush(),
            Printer::File(f) => f.flush(),
        }
    }
}

pub trait AllPrinters: RMarkdownPrinter + CvDeveloperLatexPrinter {}

impl<T: RMarkdownPrinter + CvDeveloperLatexPrinter> AllPrinters for T {}
