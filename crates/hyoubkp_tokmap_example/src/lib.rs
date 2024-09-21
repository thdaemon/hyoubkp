#[cfg(feature = "user")]
pub mod user;

use std::collections::HashMap;

use hyoubkp_base::tokmap::{TokenMapper, TokenMapperOption};
use hyoubkp_base::transaction::TransactionFactory;
use hyoubkp_base::error::Result;

#[derive(Debug)]
pub struct TokenMapperImpl {
    bank_account_tokens: Vec<&'static str>,
    expense_account_tokens: Vec<&'static str>,
}

impl TokenMapperImpl {
    pub fn new(_options: &HashMap<TokenMapperOption, String>) -> Result<TokenMapperImpl> {
        Ok(Self {
            bank_account_tokens: vec!["工行", "农行", "中行", "建行", "交行", "邮储"],
            expense_account_tokens: vec!["用餐", "杂项"],
        })
    }
}

impl TokenMapper for TokenMapperImpl {
    fn is_option_supported(_opt: TokenMapperOption) -> bool {
        false
    }

    fn register_account_tokens(&self) -> Vec<&'static str> {
        let mut v = self.bank_account_tokens.clone();
        v.extend(self.expense_account_tokens.iter());
        v
    }

    fn register_hint_tokens(&self) -> Vec<&'static str> {
        vec!["还款", "未出账单", "利息", "信用卡", "储蓄卡"]
    }

    fn fallback_account(&self) -> String {
        "不平衡的-CNY".into()
    }

    fn on_account(&self, fac: &mut TransactionFactory, account: &str) -> bool {
        match account {
            "工行" => {
                fac.set_account("资产:银行:ICBC 工商银行".into());
            }
            "农行" => {
                fac.set_account("负债:信用卡:农行 6666".into());

                if fac.is_credit() && fac.check_hint("储蓄卡") {
                    fac.set_account("资产:银行:ABC 农业银行".into());
                }
                if fac.is_debit() && !fac.check_hint("还款") {
                    fac.set_account("资产:银行:ABC 农业银行".into());
                }
            }
            "中行" => {
                self.bank_common(
                    fac,
                    "资产:银行:BOC 中国银行",
                    "负债:信用卡:中行 1234",
                    "负债:信用卡:中行 1234:已出账单",
                );
            }
            "建行" => {
                self.bank_common(
                    fac,
                    "资产:银行:CCB 建设银行",
                    "负债:信用卡:建行 8888",
                    "负债:信用卡:建行 8888:已出账单",
                );
            }
            "用餐" => {
                fac.set_account("支出:用餐".into());
            }
            "杂项" => {
                fac.set_account("支出:杂项".into());
            }
            _ => return false,
        };

        true
    }

    fn on_reward(&self, fac: &mut TransactionFactory) {
        fac.set_account("收入:优惠券变现".into());

        if fac.check_debit(&["用餐", "杂项"]) {
            fac.set_account("收入:优惠或礼遇".into());
        }
        if fac.check_hint("利息") {
            fac.set_account("收入:利息".into());
        }
    }
}

impl TokenMapperImpl {
    fn bank_common(
        &self,
        fac: &mut TransactionFactory,
        saving_acc_name: &str,
        credit_card_name: &str,
        credit_card_bill_name: &str,
    ) {
        fac.set_account(credit_card_name.into());

        if fac.is_credit() {
            if fac.check_hint("储蓄卡") || fac.check_debit(&self.bank_account_tokens) {
                fac.set_account(saving_acc_name.into());
            }
        }
        if fac.is_debit() && !fac.check_hint("还款") {
            fac.set_account(saving_acc_name.into());
        }
        if fac.is_debit() && fac.check_hint("还款") && !fac.check_hint("未出账单") {
            fac.set_account(credit_card_bill_name.into());
        }
    }
}
