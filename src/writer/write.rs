use std::fs::File;

use futures::future::join_all;

use crate::util::error::ErrorToString;


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


    pub fn add_url_to_check(&mut self, link: String) {
        self.urls.push(link);
    }

    pub async fn check_urls(&self) -> Result<(), String> {
        async fn check_url(url: &str) -> Result<(), String> {
            let status = reqwest::get(url)
                .await
                .err_str()?
                .status();

            if status == 200 {
                Ok(())
            } else {
                Err(format!("Error code {status} for URL {url}"))
            }
        }

        let results = join_all(self.urls.iter().map(|e| { check_url(e) })).await;

        let mut error = String::new();
        for result in results {
            if let Err(e) = result {
                error.push_str(&e);
                error.push('\n');
            }
        }

        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }
}
