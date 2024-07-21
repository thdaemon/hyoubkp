use crate::transaction::TransactionFactory;

pub trait TokenMapper {
    fn register_account_tokens(&self) -> Vec<&str>;
    fn register_hint_tokens(&self) -> Vec<&str>;
    fn fallback_account(&self) -> String;
    fn on_account(&self, fac: &mut TransactionFactory, account: &str) -> bool;
    fn on_reward(&self, fac: &mut TransactionFactory);
}
