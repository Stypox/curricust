pub mod latex_writer;
pub mod markdown_to_latex;

use std::{fmt::Debug, fs::File};

use self::latex_writer::LatexWriter;

// keep Stdout and Stderr for easy testing
#[allow(dead_code)]
pub enum MyWrite {
    Stdout,
    Stderr,
    File(File),
}

impl std::io::Write for MyWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            MyWrite::Stdout => std::io::stdout().write(buf),
            MyWrite::Stderr => std::io::stderr().write(buf),
            MyWrite::File(f) => f.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            MyWrite::Stdout => std::io::stdout().flush(),
            MyWrite::Stderr => std::io::stderr().flush(),
            MyWrite::File(f) => f.flush(),
        }
    }
}

pub trait AllWriters: LatexWriter + Debug {}

impl<T: LatexWriter + Debug> AllWriters for T {}
