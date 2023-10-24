use std::fs::File;

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
