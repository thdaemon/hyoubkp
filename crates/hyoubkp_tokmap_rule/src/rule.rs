use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use hyoubkp_base::error::Result;

use crate::many::Many;
use crate::{AccountToken, AccountTokenOrTag, HintToken, Tag};

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub enum UserRuleSide {
    Debit,
    Credit,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserRuleRuleEntry {
    #[serde(default)]
    pub token: Many<AccountToken>,

    #[serde(default)]
    pub side: Option<UserRuleSide>,

    #[serde(default)]
    pub hint: Many<HintToken>,

    #[serde(default)]
    pub opposite: Many<AccountTokenOrTag>,

    #[serde(default)]
    pub debit: Many<AccountTokenOrTag>,

    #[serde(default)]
    pub credit: Many<AccountTokenOrTag>,

    #[serde(default)]
    pub account: Option<String>,

    #[serde(default)]
    pub import: Vec<String>,
}

pub type UserRuleSetMap = HashMap<String, Vec<UserRuleRuleEntry>>;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserRule {
    pub fallback: String,

    #[serde(default)]
    pub hints: Vec<HintToken>,

    #[serde(default)]
    pub tags: HashMap<AccountToken, Vec<Tag>>,

    pub ruleset: UserRuleSetMap,
}

#[derive(Debug, Clone, Copy)]
pub enum CookedRuleEntryAccCheckTarget {
    NoOne,
    Debit,
    Credit,
    Opposite,
}

#[derive(Debug, Clone)]
pub struct CookedRuleEntry {
    pub side: Option<UserRuleSide>,
    pub hint_check_list: Vec<HintToken>,
    pub acc_check_target: CookedRuleEntryAccCheckTarget,
    pub acc_check_list: Vec<AccountToken>,
    pub account: String,
}

#[derive(Debug, Default)]
pub struct CookedRule {
    pub fallback: String,
    pub accounts: Vec<AccountToken>,
    pub hints: Vec<HintToken>,
    pub tags: HashMap<Tag, Vec<AccountToken>>,
    pub ruleset_main: HashMap<AccountToken, Vec<CookedRuleEntry>>,
    pub ruleset_reward: Vec<CookedRuleEntry>,
}

impl CookedRule {
    fn patch_list_of_string_for_arg<'a>(it: impl Iterator<Item = &'a mut String>, args: &[String]) {
        for s in it {
            if s.starts_with('$') {
                let idx: usize = s[1..].parse().unwrap_or(0);
                if idx > 0 && idx <= args.len() {
                    *s = args[idx - 1].clone();
                }
            }
        }
    }

    fn rec_collect_ruleset(
        ruleset_map: &UserRuleSetMap,
        name: &str
    ) -> Result<Vec<UserRuleRuleEntry>> {
        let mut result = Vec::new();

        let ruleset = ruleset_map
            .get(name)
            .ok_or_else(|| hyoubkp_base::err!("ruleset '{}' is required", name))?;

        for rule in ruleset {
            if !rule.import.is_empty() {
                let imported_rules =
                    Self::rec_collect_ruleset(ruleset_map, rule.import[0].as_str())?;

                let args = &rule.import[1..];
                result.extend(imported_rules.into_iter().map(|mut r| {
                    if !rule.token.is_empty() {
                        r.token = rule.token.clone();
                    }
                    if rule.side.is_some() {
                        r.side = rule.side;
                    }
                    if !rule.hint.is_empty() {
                        r.hint = rule.hint.clone();
                    }
                    if !rule.opposite.is_empty() {
                        r.opposite = rule.opposite.clone();
                    }
                    if !rule.debit.is_empty() {
                        r.debit = rule.debit.clone();
                    }
                    if !rule.credit.is_empty() {
                        r.credit = rule.credit.clone();
                    }
                    if rule.account.is_some() {
                        r.account = rule.account.clone();
                    }

                    Self::patch_list_of_string_for_arg(r.account.iter_mut(), args);
                    Self::patch_list_of_string_for_arg(r.hint.iter_mut(), args);
                    Self::patch_list_of_string_for_arg(r.opposite.iter_mut(), args);
                    Self::patch_list_of_string_for_arg(r.debit.iter_mut(), args);
                    Self::patch_list_of_string_for_arg(r.credit.iter_mut(), args);

                    r
                }));
            } else {
                result.push(rule.clone());
            }
        }

        Ok(result)
    }

    fn parse_ruleset(
        tags: &HashMap<Tag, Vec<AccountToken>>,
        ruleset_map: &UserRuleSetMap,
        name: &str,
        index_by_acctok: bool,
    ) -> Result<Vec<(Option<AccountToken>, CookedRuleEntry)>> {
        let mut result = Vec::new();

        for rule in Self::rec_collect_ruleset(ruleset_map, name)?.into_iter() {
            let mut entries = Vec::new();
            if rule.account.is_some() {
                if [&rule.opposite, &rule.credit, &rule.debit]
                    .iter()
                    .filter(|a| !a.is_empty())
                    .count()
                    > 1
                {
                    hyoubkp_base::bail!(
                        "'opposite', 'debit' and 'credit' conflict with each other"
                    );
                }

                let mut acc_check_target = CookedRuleEntryAccCheckTarget::NoOne;
                let mut acc_check_set = HashSet::new();

                let rule_acc_check_list = if !rule.opposite.is_empty() {
                    acc_check_target = CookedRuleEntryAccCheckTarget::Opposite;
                    Some(&rule.opposite)
                } else if !rule.credit.is_empty() {
                    acc_check_target = CookedRuleEntryAccCheckTarget::Credit;
                    Some(&rule.credit)
                } else if !rule.debit.is_empty() {
                    acc_check_target = CookedRuleEntryAccCheckTarget::Debit;
                    Some(&rule.debit)
                } else {
                    None
                };

                if let Some(rule_acc_check_list) = rule_acc_check_list {
                    for account_or_tag in rule_acc_check_list.iter() {
                        if account_or_tag.starts_with('#') {
                            let tag_name = &account_or_tag[1..];
                            acc_check_set.extend(
                                tags.get(tag_name)
                                    .ok_or_else(|| {
                                        hyoubkp_base::err!("no account marked tag '{}'", tag_name)
                                    })?
                                    .iter(),
                            )
                        } else {
                            acc_check_set.insert(account_or_tag);
                        }
                    }
                }

                let entry = CookedRuleEntry {
                    side: rule.side,
                    hint_check_list: rule.hint.iter().map(ToOwned::to_owned).collect(),
                    acc_check_target,
                    acc_check_list: acc_check_set.into_iter().map(ToOwned::to_owned).collect(),
                    account: rule.account.unwrap(),
                };

                entries.push(entry);
            } else {
                hyoubkp_base::bail!("Either 'account' or 'import' is required. Rule: {:?}", rule);
            }

            if index_by_acctok {
                for acctok in rule.token.iter() {
                    result.extend(entries.iter().map(|cr| (Some(acctok.clone()), cr.clone())));
                }
            } else {
                result.extend(entries.into_iter().map(|cr| (None, cr)));
            }
        }

        Ok(result)
    }

    pub fn load_from_file(f: &Path) -> Result<Self> {
        let rule: UserRule = toml::from_str(std::fs::read_to_string(f)?.as_str())?;

        let mut cooked = CookedRule {
            fallback: rule.fallback,
            hints: rule.hints,
            ..Default::default()
        };

        let mut accounts = HashSet::new();

        for (acc, tags) in rule.tags.into_iter() {
            accounts.insert(acc.clone());
    
            for tag in tags.into_iter() {
                cooked
                    .tags
                    .entry(tag)
                    .or_insert_with(Default::default)
                    .push(acc.clone());
            }
        }

        cooked.ruleset_main =
            Self::parse_ruleset(&cooked.tags, &rule.ruleset, "main", true)?
                .into_iter()
                .fold(HashMap::new(), |mut map, (key, value)| {
                    map.entry(key.unwrap()).or_default().push(value);
                    map
                });

        cooked.ruleset_reward =
            Self::parse_ruleset(&cooked.tags, &rule.ruleset, "reward", false)?
                .into_iter()
                .map(|(_, value)| value)
                .collect();

        for (k, v) in cooked.ruleset_main.iter() {
            accounts.insert(k.clone());
            for r in v.iter() {
                for t in r.acc_check_list.iter() {
                    accounts.insert(t.clone());
                }
            }
        }

        for r in cooked.ruleset_reward.iter() {
            for t in r.acc_check_list.iter() {
                accounts.insert(t.clone());
            }
        }

        cooked.accounts = accounts.into_iter().collect();

        Ok(cooked)
    }
}
