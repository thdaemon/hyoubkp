use std::collections::HashMap;

use crate::{
    date::Date,
    expr::{Expr, ExprCreditPrice},
    price::Price,
    tokmap::TokenMapper,
    HintToken, Shares,
};

#[derive(Default, Debug)]
pub struct Transaction {
    pub has_build_error: bool,
    pub date: Date,
    pub num_base: u32,
    pub debit_entries: Vec<Entry>,
    pub credit_entries: Vec<Entry>,
    pub description: Option<String>,
    pub orig_expr: Option<String>,
}

#[derive(Debug)]
pub struct Entry {
    pub account: String,
    pub amount: Amount,
}

#[derive(Debug)]
pub enum Amount {
    Shares(Shares, Price),
    Price(Price),
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amount::Shares(_, _) => todo!(),
            Amount::Price(p) => write!(f, "{}", p),
        }
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(expr) = &self.orig_expr {
            writeln!(f, "Expression: {}", expr)?;
        }
        writeln!(f, "Date: {}, num base: {}", self.date, self.num_base)?;
        writeln!(
            f,
            "Transaction desc: {}",
            self.description.as_deref().unwrap_or_default()
        )?;
        for e in self.debit_entries.iter() {
            writeln!(f, "{} debit {}", e.account, e.amount)?
        }
        for e in self.credit_entries.iter() {
            writeln!(f, "{} credit {}", e.account, e.amount)?
        }

        Ok(())
    }
}

impl Transaction {
    pub fn is_empty(&self) -> bool {
        self.credit_entries.is_empty() && self.debit_entries.is_empty()
    }
}

#[derive(Default, Debug)]
pub struct TransactionFactory {
    transaction: Transaction,

    credit_tok: Option<String>,
    debit_tok: Option<String>,
    credit_account: Option<String>,
    debit_account: Option<String>,
    hints_map: HashMap<HintToken, bool>,

    current_account: Option<String>,
}

//#[derive(Default, Debug)]
//struct TransactionFactoryAccount {
//    account: String,
//    tok: String,
//}
//
//impl TransactionFactoryAccount {
//    fn new(account: String, tok: &str) -> Self {
//        Self {
//            account,
//            tok: tok.to_string(),
//        }
//    }
//}
//
//impl std::ops::Deref for TransactionFactoryAccount {
//    type Target = str;
//
//    fn deref(&self) -> &Self::Target {
//        self.account.deref()
//    }
//}

impl TransactionFactory {
    pub fn set_account(&mut self, account: String) {
        self.current_account = Some(account);
    }

    pub fn is_credit(&self) -> bool {
        self.credit_account.is_none()
    }

    pub fn is_debit(&self) -> bool {
        !self.is_credit() && self.debit_account.is_none()
    }

    pub fn check_hint(&mut self, hint: &str) -> bool {
        let hint = self.hints_map.get_mut(hint);
        match hint {
            Some(hint) => {
                *hint = true;
                true
            }
            None => false,
        }
    }

    pub fn remove_hint(&mut self, hint: &str) {
        self.hints_map.remove(hint);
    }

    pub fn check_credit(&self, account: &[&str]) -> bool {
        account
            .iter()
            .any(|a| *a == self.credit_tok.as_deref().unwrap_or_default())
    }

    pub fn check_debit(&self, account: &[&str]) -> bool {
        account
            .iter()
            .any(|a| *a == self.debit_tok.as_deref().unwrap_or_default())
    }

    pub fn check_opposite(&self, account: &[&str]) -> bool {
        if self.is_debit() {
            self.check_credit(account)
        } else if self.is_credit() {
            self.check_debit(account)
        } else {
            panic!()
        }
    }

    fn map_account(&mut self, cr_or_dr: u32, token_mapper: &impl TokenMapper, tok: &str) {
        self.current_account = None;

        if token_mapper.on_account(self, tok) {
            if let Some(account) = std::mem::take(&mut self.current_account) {
                if cr_or_dr == 1 {
                    self.credit_account = Some(account);
                } else {
                    self.debit_account = Some(account);
                }
            }
        } else {
            //todo!()
        }
    }

    pub fn set_expr(&mut self, token_mapper: &impl TokenMapper, expr: &Expr) {
        self.hints_map
            .iter_mut()
            .for_each(|(_, accessed)| *accessed = false);

        self.hints_map
            .extend(expr.hints.iter().map(|i| (i.clone(), false)));

        let mut credit_tok = None;
        let mut debit_tok = None;
        for (i, at) in expr.accounts.iter().enumerate() {
            if i == 0 {
                self.credit_account = None;
                self.credit_tok = Some(at.clone());
                credit_tok = Some(at.as_str());
            } else if i == 1 {
                self.debit_account = None;
                self.debit_tok = Some(at.clone());
                debit_tok = Some(at.as_str());
            }
        }

        if let Some(tok) = credit_tok {
            self.map_account(1, token_mapper, tok);
        }
        if let Some(tok) = debit_tok {
            self.map_account(2, token_mapper, tok);
        }

        for trans in expr.trans.iter() {
            if trans.cash_backs.is_empty() {
                if self.debit_account.is_none() || self.credit_account.is_none() {
                    self.transaction.has_build_error = true;
                }
            } else {
                if self.credit_account.is_none() {
                    self.transaction.has_build_error = true;
                }
            }

            for _ in 0..trans.multiple {
                if !trans.is_cashback_only() {
                    self.transaction.debit_entries.push(Entry {
                        account: self
                            .debit_account
                            .as_ref()
                            .map(|a| a.clone())
                            .unwrap_or_else(|| token_mapper.fallback_account()),
                        amount: Amount::Price(trans.price_debit),
                    });

                    if trans.price_credit_chain.is_empty() {
                        self.transaction.credit_entries.push(Entry {
                            account: self
                                .credit_account
                                .as_ref()
                                .map(|a| a.clone())
                                .unwrap_or_else(|| token_mapper.fallback_account()),
                            amount: Amount::Price(trans.price_debit),
                        });
                    } else {
                        let mut price = trans.price_debit.clone();

                        for pc in trans.price_credit_chain.iter() {
                            let reward;

                            match pc {
                                ExprCreditPrice::Reward(r) => {
                                    reward = r.clone();
                                    price = price - reward;
                                }
                                ExprCreditPrice::Credit(c) => {
                                    reward = price - *c;
                                    price = c.clone();
                                }
                            }

                            self.current_account = None;
                            token_mapper.on_reward(self);
                            self.transaction.credit_entries.push(Entry {
                                account: self
                                    .current_account
                                    .to_owned()
                                    .unwrap_or_else(|| token_mapper.fallback_account()),
                                amount: Amount::Price(reward),
                            });
                        }

                        self.transaction.credit_entries.push(Entry {
                            account: self
                                .credit_account
                                .as_ref()
                                .map(|a| a.clone())
                                .unwrap_or_else(|| token_mapper.fallback_account()),
                            amount: Amount::Price(price),
                        });
                    }
                }

                for e in trans.cash_backs.iter() {
                    self.current_account = None;
                    token_mapper.on_reward(self);
                    self.transaction.credit_entries.push(Entry {
                        account: self
                            .current_account
                            .to_owned()
                            .unwrap_or_else(|| token_mapper.fallback_account()),
                        amount: Amount::Price(e.clone()),
                    });

                    self.transaction.debit_entries.push(Entry {
                        account: self
                            .credit_account
                            .as_ref()
                            .map(|a| a.clone())
                            .unwrap_or_else(|| token_mapper.fallback_account()),
                        amount: Amount::Price(e.clone()),
                    });
                }
            }
        }
    }

    pub fn build(mut self, token_mapper: &impl TokenMapper) -> Transaction {
        if (!self.transaction.has_build_error
            && self.hints_map.iter().any(|(_, accessed)| !accessed))
            || self.transaction.is_empty()
        {
            self.transaction.has_build_error = true;
            self.transaction.credit_entries.push(Entry {
                account: token_mapper.fallback_account(),
                amount: Amount::Price(Price::default()),
            });
        }

        self.transaction
    }
}
