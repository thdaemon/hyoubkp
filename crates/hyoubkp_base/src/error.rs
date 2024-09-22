use std::backtrace::Backtrace;
use std::error::Error as StdError;
use std::fmt::{Debug, Display};

pub type Result<T> = std::result::Result<T, Error>;

trait IntoHyoubkpError: StdError {}
impl IntoHyoubkpError for std::io::Error {}
impl IntoHyoubkpError for std::num::ParseIntError {}
#[cfg(feature = "toml")]
impl IntoHyoubkpError for toml::de::Error {}

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub backtrace: Backtrace,
}

impl StdError for Error {}

impl Error {
    pub fn new(id: Option<&'static str>, s: impl AsRef<str>) -> Self {
        Self {
            message: format!("{}: {}", id.unwrap_or("Hyoubkp error"), s.as_ref()),
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n\nBacktrace:\n{}", self.message, self.backtrace)
    }
}

impl<E> From<E> for Error
where
    E: IntoHyoubkpError,
{
    fn from(error: E) -> Self {
        Self::new(Some(std::any::type_name::<E>()), error.to_string())
    }
}

#[macro_export]
macro_rules! err {
    ($msg:literal $(,)?) => {
        $crate::error::Error::new(None, $msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::Error::new(None, format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return Err($crate::err!($msg))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::err!($fmt, $($arg)*))
    };
}

pub(crate) use bail;
//pub(crate) use err;
