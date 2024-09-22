pub mod many;
pub mod rule;

use std::collections::HashMap;
use std::path::Path;

use hyoubkp_base::error::Result;
use hyoubkp_base::{
    tokmap::{TokenMapper, TokenMapperOption},
    transaction::TransactionFactory,
};
use rule::*;

pub type Tag = String;
pub type AccountTokenOrTag = String;
pub use hyoubkp_base::{AccountToken, HintToken};

#[derive(Debug)]
pub struct TokenMapperImpl {
    rule: CookedRule,
}

impl TokenMapperImpl {
    pub fn new(options: &HashMap<TokenMapperOption, String>) -> Result<TokenMapperImpl> {
        let rule = match options.get(&TokenMapperOption::RuleFile) {
            Some(f) => CookedRule::load_from_file(Path::new(f))?,
            None => hyoubkp_base::bail!("rule-file tokmap option is required"),
        };

        Ok(Self { rule })
    }
}

impl TokenMapper for TokenMapperImpl {
    fn get_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn is_option_supported(opt: TokenMapperOption) -> bool {
        match opt {
            TokenMapperOption::RuleFile => true,
            _ => false,
        }
    }

    fn register_account_tokens(&self) -> Vec<&str> {
        self.rule.accounts.iter().map(AsRef::as_ref).collect()
    }

    fn register_hint_tokens(&self) -> Vec<&str> {
        self.rule.hints.iter().map(AsRef::as_ref).collect()
    }

    fn fallback_account(&self) -> String {
        self.rule.fallback.clone()
    }

    fn on_account(&self, fac: &mut TransactionFactory, account: &str) -> bool {
        match self.rule.ruleset_main.get(account) {
            Some(rules) => self.check_rules(fac, rules),
            None => false,
        }
    }

    fn on_reward(&self, fac: &mut TransactionFactory) {
        if !self.check_rules(fac, &self.rule.ruleset_reward) {
            fac.set_account(self.rule.fallback.clone());
        }
    }
}

impl TokenMapperImpl {
    fn check_rules(&self, fac: &mut TransactionFactory, rules: &[CookedRuleEntry]) -> bool {
        let mut matched = false;

        'rule_loop: for rule in rules.iter() {
            if rule.side.is_some() {
                match rule.side.unwrap() {
                    UserRuleSide::Debit => {
                        if !fac.is_debit() {
                            continue;
                        }
                    }
                    UserRuleSide::Credit => {
                        if !fac.is_credit() {
                            continue;
                        }
                    }
                }
            }

            let check_list: Vec<&str> = rule.acc_check_list.iter().map(String::as_ref).collect();
            match rule.acc_check_target {
                CookedRuleEntryAccCheckTarget::NoOne => (),
                CookedRuleEntryAccCheckTarget::Debit => {
                    if !fac.check_debit(&check_list) {
                        continue;
                    }
                }
                CookedRuleEntryAccCheckTarget::Credit => {
                    if !fac.check_credit(&check_list) {
                        continue;
                    }
                }
                CookedRuleEntryAccCheckTarget::Opposite => {
                    if !fac.check_opposite(&check_list) {
                        continue;
                    }
                }
            }

            for hint in rule.hint_check_list.iter() {
                if !fac.check_hint(hint) {
                    continue 'rule_loop;
                }
            }

            fac.set_account(rule.account.clone());
            matched = true;
            break;
        }

        matched
    }
}
