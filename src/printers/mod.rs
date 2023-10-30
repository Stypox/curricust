pub mod latex_printer;
pub mod markdown_to_latex;

use std::{fmt::Debug, fs::File};

use self::latex_printer::LatexPrinter;

// keep Stdout and Stderr for easy testing
#[allow(dead_code)]
pub enum Writer {
    Stdout,
    Stderr,
    File(File),
}

impl std::io::Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Writer::Stdout => std::io::stdout().write(buf),
            Writer::Stderr => std::io::stderr().write(buf),
            Writer::File(f) => f.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Writer::Stdout => std::io::stdout().flush(),
            Writer::Stderr => std::io::stderr().flush(),
            Writer::File(f) => f.flush(),
        }
    }
}

pub trait AllPrinters: LatexPrinter + Debug {}

impl<T: LatexPrinter + Debug> AllPrinters for T {}
