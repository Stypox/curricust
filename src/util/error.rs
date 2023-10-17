use std::fmt::Display;

pub trait ErrorToString<T> {
    fn err_str(self) -> Result<T, String>;
}

impl<T, E> ErrorToString<T> for Result<T, E>
where
    E: Display,
{
    fn err_str(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}
