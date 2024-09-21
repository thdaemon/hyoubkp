use crate::transaction::TransactionFactory;

pub trait TokenMapper {
    fn is_option_supported(opt: TokenMapperOption) -> bool;
    fn register_account_tokens(&self) -> Vec<&str>;
    fn register_hint_tokens(&self) -> Vec<&str>;
    fn fallback_account(&self) -> String;
    fn on_account(&self, fac: &mut TransactionFactory, account: &str) -> bool;
    fn on_reward(&self, fac: &mut TransactionFactory);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum TokenMapperOption {
    RuleFile,
}

impl TokenMapperOption {
    pub const OPTIONS: [TokenMapperOption; 1] = [ Self::RuleFile ];

    pub fn description(&self) -> &'static str {
        match self {
            Self::RuleFile => "rule-file=<file path> - specify rule files for tokmap",
        }
    }
}