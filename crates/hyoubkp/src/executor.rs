use std::collections::HashMap;

use crate::parser::{parse_fail, ParseError, ParseResult, Parser};
use crate::tokmap::{tokmap_dispatch, TokenMapperDispatch, TokenMapperKind};
use hyoubkp_base::date::Date;
use hyoubkp_base::tokmap::{TokenMapper, TokenMapperOption};
use hyoubkp_base::transaction::Transaction;
use hyoubkp_base::transaction::TransactionFactory;
use hyoubkp_base::error::Result;

#[derive(Debug)]
pub struct Executor {
    pub token_mapper: TokenMapperDispatch,
    pub parser: Parser,

    date: ExecutorDate,
    num_base: u32,
}

impl Executor {
    pub fn new(tokmap_kind: TokenMapperKind, options: &HashMap<TokenMapperOption, String>) -> Result<Self> {
        let token_mapper = TokenMapperDispatch::new(tokmap_kind, options)?;
        let parser = Parser::new(
            &tokmap_dispatch!(tm, &token_mapper, tm.register_account_tokens()),
            &tokmap_dispatch!(tm, &token_mapper, tm.register_hint_tokens()),
        );

        Ok(Self {
            token_mapper,
            parser,
            date: ExecutorDate::default(),
            num_base: 0,
        })
    }

    pub fn parse_expr(&mut self, expr: impl AsRef<str>) -> ParseResult<Transaction> {
        self.parser.reset();
        let cexpr = self.parser.parse_expr(expr.as_ref())?;

        let mut factory: TransactionFactory = TransactionFactory::default();

        if cexpr.exprs.is_empty() && cexpr.comment.is_none() {
            parse_fail!('\0', 0, "Expression can not be parsed")
        }

        for expr in cexpr.exprs.iter() {
            tokmap_dispatch!(tm, &self.token_mapper, factory.set_expr(tm, expr))
        }

        let mut transaction = tokmap_dispatch!(tm, &self.token_mapper, factory.build(tm));

        transaction.date = self.date.get_date();
        transaction.num_base = self.num_base;
        transaction.orig_expr = Some(expr.as_ref().to_string());

        if transaction.has_build_error {
            transaction.description = Some(
                cexpr.comment.unwrap_or_default() + " FIXME:[" + expr.as_ref() + "]",
            );
        } else {
            transaction.description = Some(cexpr.comment.unwrap_or_default());
        }

        Ok(transaction)
    }

    pub fn parse_directive(
        &mut self,
        directive: impl AsRef<str>,
    ) -> Result<()> {
        let directive = directive.as_ref();

        if let Some(date) = directive.strip_prefix(".date ") {
            self.date = ExecutorDate::Fixed(Date::from(date));
        }

        if let Some(num) = directive.strip_prefix(".num ") {
            self.num_base = num.parse()?;
        }

        Ok(())
    }

    pub fn enable_realtime_date(&mut self) {
        self.date = ExecutorDate::Realtime;
    }

    pub fn get_tokmap_version(&self) -> &'static str {
        tokmap_dispatch!(tm, &self.token_mapper, tm.get_version())
    }
}

#[derive(Debug)]
pub enum ExecutorDate {
    Fixed(Date),
    Realtime,
}

impl Default for ExecutorDate {
    fn default() -> Self {
        Self::Fixed(Date::today())
    }
}

impl ExecutorDate {
    pub fn get_date(&self) -> Date {
        match self {
            ExecutorDate::Fixed(d) => d.clone(),
            ExecutorDate::Realtime => Date::today(),
        }
    }
}