use hyoubkp_base::{datagen::DataGen, transaction::Transaction};

#[cfg(feature = "datagen_gnucash")]
pub use hyoubkp_datagen_gnucash as datagen_impl_gnucash;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[repr(u8)]
pub enum DataGenKind {
    Str = 0,
    #[cfg(feature = "datagen_gnucash")]
    #[cfg_attr(feature = "clap", clap(name = "gnucash"))]
    GnuCash = 1,
}

#[derive(Debug)]
pub enum DataGenDispatch {
    Str(DataGenImplStr),
    #[cfg(feature = "datagen_gnucash")]
    GnuCash(datagen_impl_gnucash::DataGenImpl),
}

impl DataGenDispatch {
    pub fn new(kind: DataGenKind) -> Self {
        match kind {
            DataGenKind::Str => Self::Str(DataGenImplStr {}),
            #[cfg(feature = "datagen_gnucash")]
            DataGenKind::GnuCash => Self::GnuCash(datagen_impl_gnucash::DataGenImpl::new()),
        }
    }
}

macro_rules! dispatch {
    ($self:ident, $n:ident, $($e:tt)*) => {
        match $self {
            DataGenDispatch::Str($n) => $($e)*,
            #[cfg(feature = "datagen_gnucash")]
            DataGenDispatch::GnuCash($n) => $($e)*,
        }
    };
}

impl DataGen for DataGenDispatch {
    fn gen_to_string(
        &self,
        transactions: &[Transaction],
        number: u32,
    ) -> Result<String, std::io::Error> {
        dispatch!(self, d, d.gen_to_string(transactions, number))
    }

    fn gen_to_bytes(
        &self,
        v: &mut Vec<u8>,
        transactions: &[Transaction],
        number: u32,
    ) -> Result<(), std::io::Error> {
        dispatch!(self, d, d.gen_to_bytes(v, transactions, number))
    }

    fn write_to(
        &self,
        f: impl std::io::Write,
        transactions: &[Transaction],
        number: u32,
    ) -> Result<(), std::io::Error> {
        dispatch!(self, d, d.write_to(f, transactions, number))
    }
}

#[derive(Debug)]
pub struct DataGenImplStr;

impl DataGen for DataGenImplStr {
    fn write_to(
        &self,
        mut f: impl std::io::Write,
        transactions: &[Transaction],
        _number: u32,
    ) -> Result<(), std::io::Error> {
        for transaction in transactions.iter() {
            writeln!(f, "{}", transaction)?;
        }

        Ok(())
    }
}