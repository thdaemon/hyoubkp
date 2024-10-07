use csv::WriterBuilder;
use hyoubkp_base::{datagen::DataGen, transaction::{Amount, Transaction}};
use uuid::Uuid;

#[derive(serde::Serialize)]
struct GnuCashCSVRow {
    date: String,
    transaction_id: String,
    number: u32,
    description: String,
    reconcile: String,
    full_account_name: String,
    amount_num: String,
    value_num: String,
}

#[derive(Debug)]
pub struct DataGenImpl;

impl DataGenImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl DataGen for DataGenImpl {
    fn write_to(
        &self,
        f: impl std::io::Write,
        transactions: &[Transaction],
        number: u32,
    ) -> Result<(), std::io::Error> {
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(f);
        //let mut wtr = Writer::from_writer(f);

        if number == 0 {
            wtr.write_record(&[
                "Date",
                "Transaction ID",
                "Number",
                "Description",
                "Reconcile",
                "Full Account Name",
                "Amount Num.",
                "Value Num.",
            ])?;
        }

        for trans in transactions.iter() {
            let transaction_id = Uuid::new_v4().as_simple().to_string();

            for (e, dc) in trans
                .credit_entries
                .iter()
                .map(|x| (x, 2))
                .chain(trans.debit_entries.iter().map(|x| (x, 1)))
            {
                let mut description = trans.description.clone().unwrap_or_default();
                if description == "" {
                    description = String::from(" ");
                }

                if dc == 1 {
                    wtr.serialize(GnuCashCSVRow {
                        date: trans.date.to_string(),
                        transaction_id: transaction_id.clone(),
                        number: number + trans.num_base,
                        description: description,
                        reconcile: String::from("n"),
                        full_account_name: e.account.clone(),
                        amount_num: Self::amount_to_amount(&e.amount),
                        value_num: Self::amount_to_value(&e.amount),
                    })?;
                } else if dc == 2 {
                    wtr.serialize(GnuCashCSVRow {
                        date: trans.date.to_string(),
                        transaction_id: transaction_id.clone(),
                        number: number + trans.num_base,
                        description: description,
                        reconcile: String::from("n"),
                        full_account_name: e.account.clone(),
                        amount_num: format!("-{}", Self::amount_to_amount(&e.amount)),
                        value_num: format!("-{}", Self::amount_to_value(&e.amount)),
                    })?;
                }
            }
        }

        Ok(())
    }
}

impl DataGenImpl {
    fn amount_to_amount(amount: &Amount) -> String {
        match amount {
            Amount::Shares(s, _) => s.to_string(),
            Amount::Price(p) => p.to_string(),
        }
    }

    fn amount_to_value(amount: &Amount) -> String {
        match amount {
            Amount::Shares(_, p) => p.to_string(),
            Amount::Price(p) => p.to_string(),
        }
    }
}