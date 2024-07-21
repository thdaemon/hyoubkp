use crate::transaction::Transaction;

pub trait DataGen {
    fn gen_to_string(
        &self,
        transactions: &[Transaction],
        number: u32,
    ) -> Result<String, std::io::Error> {
        let mut bytes = Vec::new();
        self.gen_to_bytes(&mut bytes, transactions, number)?;
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }

    fn gen_to_bytes(
        &self,
        v: &mut Vec<u8>,
        transactions: &[Transaction],
        number: u32,
    ) -> Result<(), std::io::Error> {
        let buf = std::io::BufWriter::new(v);
        self.write_to(buf, transactions, number)
    }

    fn write_to(
        &self,
        f: impl std::io::Write,
        transactions: &[Transaction],
        number: u32,
    ) -> Result<(), std::io::Error>;
}