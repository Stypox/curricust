use std::fs::File;


pub struct MyWrite {
    urls: Vec<String>,
    handle: MyWriteHandle,
}

enum MyWriteHandle {
    Stdout,
    Stderr,
    File(File),
}

impl std::io::Write for MyWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.handle {
            MyWriteHandle::Stdout => std::io::stdout().write(buf),
            MyWriteHandle::Stderr => std::io::stderr().write(buf),
            MyWriteHandle::File(f) => f.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match &mut self.handle {
            MyWriteHandle::Stdout => std::io::stdout().flush(),
            MyWriteHandle::Stderr => std::io::stderr().flush(),
            MyWriteHandle::File(f) => f.flush(),
        }
    }
}

impl MyWrite {
    pub fn add_url_to_check(&mut self, link: String) {
        self.urls.push(link);
    }

    #[allow(dead_code)] // keep for easy testing
    pub fn stdout() -> Self {
        Self { urls: vec![], handle: MyWriteHandle::Stdout }
    }

    #[allow(dead_code)] // keep for easy testing
    pub fn stderr() -> Self {
        Self { urls: vec![], handle: MyWriteHandle::Stderr }
    }

    pub fn file(f: File) -> Self {
        Self { urls: vec![], handle: MyWriteHandle::File(f) }
    }
}
