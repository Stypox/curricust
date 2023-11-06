pub mod latex_writer;
pub mod markdown_to_latex;

use std::{fmt::Debug, fs::File};

use self::latex_writer::LatexWriter;

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

pub trait AllWriters: LatexWriter + Debug {}

impl<T: LatexWriter + Debug> AllWriters for T {}
