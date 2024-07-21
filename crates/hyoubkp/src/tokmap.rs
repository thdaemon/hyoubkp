#[cfg(feature = "tokmap_example")]
pub use hyoubkp_tokmap_example as tokmap_impl_example;

#[cfg(feature = "tokmap_user")]
pub use hyoubkp_tokmap_example::user as tokmap_impl_user;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[repr(u8)]
pub enum TokenMapperKind {
    #[cfg(feature = "tokmap_example")]
    Example = 0,
    #[cfg(feature = "tokmap_user")]
    User = 1,
    #[cfg(feature = "tokmap_ffi")]
    FFI = 2,
    #[cfg(feature = "tokmap_python")]
    Python = 3,
}

#[derive(Debug)]
pub enum TokenMapperDispatch {
    #[cfg(feature = "tokmap_example")]
    Example(tokmap_impl_example::TokenMapperImpl),
    #[cfg(feature = "tokmap_user")]
    User(tokmap_impl_user::TokenMapperImpl),
}

impl TokenMapperDispatch {
    pub fn new(kind: TokenMapperKind) -> Self {
        match kind {
            #[cfg(feature = "tokmap_example")]
            TokenMapperKind::Example => Self::Example(tokmap_impl_example::TokenMapperImpl::new()),
            #[cfg(feature = "tokmap_user")]
            TokenMapperKind::User => Self::User(tokmap_impl_user::TokenMapperImpl::new()),
        }
    }
}

macro_rules! tokmap_dispatch {
    ($n:ident, $v:expr, $($e:tt)*) => {
        match $v {
            #[cfg(feature = "tokmap_example")]
            TokenMapperDispatch::Example($n) => $($e)*,
            #[cfg(feature = "tokmap_user")]
            TokenMapperDispatch::User($n) => $($e)*,
        }
    };
}

pub(crate) use tokmap_dispatch;