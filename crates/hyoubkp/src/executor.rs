use crate::parser::{parse_fail, ParseError, ParseResult, Parser};
use crate::tokmap::{tokmap_dispatch, TokenMapperDispatch, TokenMapperKind};
use hyoubkp_base::date::Date;
use hyoubkp_base::tokmap::TokenMapper;
use hyoubkp_base::transaction::Transaction;
use hyoubkp_base::transaction::TransactionFactory;

#[derive(Debug)]
pub struct Executor {
    pub token_mapper: TokenMapperDispatch,
    pub parser: Parser,

    date: Date,
}

impl Executor {
    pub fn new(tokmap_kind: TokenMapperKind) -> Self {
        let token_mapper = TokenMapperDispatch::new(tokmap_kind);
        let parser = Parser::new(
            &tokmap_dispatch!(tm, &token_mapper, tm.register_account_tokens()),
            &tokmap_dispatch!(tm, &token_mapper, tm.register_hint_tokens()),
        );

        Self {
            token_mapper,
            parser,
            date: Date::today()
        }
    }

    pub fn parse_expr(&mut self, expr: impl AsRef<str>) -> ParseResult<Transaction> {
        let cexpr = self.parser.parse_expr(expr.as_ref())?;
        self.parser.reset();

        let mut factory: TransactionFactory = TransactionFactory::default();

        if cexpr.exprs.is_empty() && cexpr.comment.is_none() {
            parse_fail!('\0', 0, "Expression can not be parsed")
        }

        for expr in cexpr.exprs.iter() {
            tokmap_dispatch!(tm, &self.token_mapper, factory.set_expr(tm, expr))
        }

        let mut transaction = tokmap_dispatch!(tm, &self.token_mapper, factory.build(tm));

        transaction.date = self.date;

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
    ) -> Result<(), hyoubkp_base::error::Error> {
        let directive = directive.as_ref();

        if let Some(date) = directive.strip_prefix(".date ") {
            self.date = Date::from(date);
        }

        Ok(())
    }
}
