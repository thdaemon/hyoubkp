use crate::{price::Price, AccountToken, HintToken};

#[derive(Default, Debug)]
pub struct CompoundExpr {
    pub exprs: Vec<Expr>,
    pub comment: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct Expr {
    pub accounts: Vec<AccountToken>,
    pub hints: Vec<HintToken>,
    pub trans: Vec<ExprTrans>,
}

impl Expr {
    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty() && self.hints.is_empty() && self.trans.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct ExprTrans {
    pub shares: Option<Price>,
    pub price_debit: Price,
    pub price_credit_chain: Vec<ExprCreditPrice>,
    pub cash_backs: Vec<Price>,
    pub multiple: u32,
}

#[derive(Debug, Clone)]
pub enum ExprCreditPrice {
    Reward(Price),
    Credit(Price),
}

impl Default for ExprTrans {
    fn default() -> Self {
        Self {
            shares: None,
            price_debit: Price::default(),
            price_credit_chain: Vec::new(),
            cash_backs: Vec::new(),
            multiple: 1,
        }
    }
}

impl ExprTrans {
    pub fn is_empty(&self) -> bool {
        self.shares.is_none()
            && self.price_debit == Price::default()
            && self.price_credit_chain.is_empty()
            && self.cash_backs.is_empty()
            && self.multiple == 1
    }

    pub fn is_valid(&self) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.price_debit.as_raw() <= 0 && self.cash_backs.is_empty() {
            return false;
        }

        let mut credit_price = self.price_debit.clone();
        for i in self.price_credit_chain.iter() {
            match i {
                ExprCreditPrice::Reward(r) => {
                    if r.as_raw() > credit_price.as_raw() {
                        return false;
                    }
                    credit_price = credit_price - *r
                }
                ExprCreditPrice::Credit(c) => {
                    if c.as_raw() > credit_price.as_raw() {
                        return false;
                    }
                    credit_price = c.clone()
                }
            }
        }

        true
    }

    pub fn is_cashback_only(&self) -> bool {
        !self.cash_backs.is_empty()
            && self.price_credit_chain.is_empty()
            && self.price_debit == Price::default()
    }
}

#[derive(Debug, Clone)]
pub enum ExprWeakAccount {
    Credit(AccountToken),
    Debit(AccountToken)
}
