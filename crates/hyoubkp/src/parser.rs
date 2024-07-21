use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;

use hyoubkp_base::expr::*;
use hyoubkp_base::price::Price;

#[derive(Clone)]
pub struct UnsafeNodeRef<T> {
    node: *mut T,
}

impl<T> From<&mut T> for UnsafeNodeRef<T> {
    fn from(value: &mut T) -> Self {
        Self {
            node: value as *mut T,
        }
    }
}

impl<T> Deref for UnsafeNodeRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.node }
    }
}

impl<T> DerefMut for UnsafeNodeRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.node }
    }
}

impl<T> Debug for UnsafeNodeRef<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnsafeNodeRef")
            .field("addr", &(self.node as usize))
            .field("raw", &*self)
            .finish()
    }
}

#[derive(Default, Clone, Debug)]
pub struct Node {
    pub ch: char,
    pub kind: NodeKind,
    pub next: Vec<Node>,
}

#[derive(Clone, Copy, Debug)]
pub enum NodeKind {
    Staging,
    AccountToken,
    HintToken,
}

impl Default for NodeKind {
    fn default() -> Self {
        Self::Staging
    }
}

impl Node {
    pub fn unsafe_ref(&mut self) -> UnsafeNodeRef<Self> {
        UnsafeNodeRef::from(self)
    }

    pub fn find(&self, ch: char) -> Option<&Node> {
        for i in self.next.iter() {
            if i.ch == ch {
                return Some(i);
            }
        }

        None
    }

    pub fn find_mut(&mut self, ch: char) -> Option<&mut Node> {
        for i in self.next.iter_mut() {
            if i.ch == ch {
                return Some(i);
            }
        }

        None
    }

    pub fn push_node(&mut self, n: Node) -> UnsafeNodeRef<Self> {
        self.next.push(n);
        self.next.last_mut().unwrap().into()
    }

    pub fn feed(&mut self, s: &str, kind: NodeKind) {
        let mut node = UnsafeNodeRef::from(self);

        for ch in s.chars() {
            match node.find_mut(ch) {
                Some(nn) => {
                    node = UnsafeNodeRef::from(nn);
                }
                None => {
                    node = node.push_node(Node {
                        ch: ch,
                        ..Default::default()
                    });
                }
            }
        }

        node.kind = kind;
    }
}

#[derive(Default, Debug)]
pub struct Parser {
    tree: Node,
    state: State,
}

#[derive(Default, Debug)]
pub struct State {
    estate: ExprState,
    pstate: PriceState,
    pbitset: u8,

    staging_token: String,
    ch: char,
    pos: usize,
}

#[derive(Clone, Debug)]
pub enum ExprState {
    Swap,
    ExprPartAccAndHint(UnsafeNodeRef<Node>),
    ExprPartPrice,
    // ExprComment,
}

impl Default for ExprState {
    fn default() -> Self {
        Self::Swap
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum PriceState {
    Debit = 0b1,
    Credit = 0b10,
    Reward = 0b100,
    Shares = 0b1000,
    Multiple = 0b10000,
    Cashback = 0b100000,
}

pub static PRICE_STATE_REENTRANT_MASK: u8 = 0b100110;
pub static PRICE_STATE_ALL_MASK: u8 = 0b111111;

impl Default for PriceState {
    fn default() -> Self {
        Self::Debit
    }
}

impl Parser {
    pub fn new<A, H>(account_toks: &[A], hint_toks: &[H]) -> Self
    where
        A: AsRef<str>,
        H: AsRef<str>,
    {
        let mut tree = Node::default();

        for i in account_toks.iter() {
            tree.feed(i.as_ref(), NodeKind::AccountToken);
        }

        for i in hint_toks.iter() {
            tree.feed(i.as_ref(), NodeKind::HintToken);
        }

        Self {
            tree: tree,
            ..Default::default()
        }
    }

    fn begin_acc_or_hint(&mut self, ch: Option<char>) -> bool {
        self.state.estate = ExprState::ExprPartAccAndHint(self.tree.unsafe_ref());

        if let Some(ch) = ch {
            match self.tree.find_mut(ch) {
                Some(nn) => {
                    self.state.staging_token.push(ch);
                    self.state.estate = ExprState::ExprPartAccAndHint(nn.unsafe_ref());
                }
                None => return false,
            };
        }

        true
    }

    fn pop_acc_or_hint_token(
        &mut self,
        node: UnsafeNodeRef<Node>,
        out_expr: &mut Expr,
    ) -> ParseResult<()> {
        if self.state.staging_token.is_empty() {
            parse_fail!(self, "Account or hint is required",);
        }

        match node.kind {
            NodeKind::Staging => parse_fail!(
                self,
                "Token '{}' is not an known account or hint",
                self.state.staging_token,
            ),
            NodeKind::AccountToken => out_expr
                .accounts
                .push(std::mem::take(&mut self.state.staging_token)),
            NodeKind::HintToken => out_expr
                .hints
                .push(std::mem::take(&mut self.state.staging_token)),
        }

        self.state.staging_token.clear();
        self.state.estate = ExprState::ExprPartAccAndHint(self.tree.unsafe_ref());

        Ok(())
    }

    fn pop_price_token(&mut self, out_trans: &mut ExprTrans) -> ParseResult<()> {
        let value = self.state.staging_token.parse::<Price>().map_err(|e| {
            parse_error!(
                self,
                "Can not parse price '{}' ({})",
                self.state.staging_token,
                e
            )
        })?;

        match self.state.pstate {
            PriceState::Debit => out_trans.price_debit = value,
            PriceState::Credit => out_trans
                .price_credit_chain
                .push(ExprCreditPrice::Credit(value)),
            PriceState::Reward => out_trans
                .price_credit_chain
                .push(ExprCreditPrice::Reward(value)),
            PriceState::Shares => todo!(),
            PriceState::Multiple => {
                if value.fractional_part() != 0 {
                    parse_fail!(
                        self,
                        "Multiple part should be an integer, but currently is '{}'",
                        self.state.staging_token
                    );
                }

                out_trans.multiple = value.integer_part() as u32;
            }
            PriceState::Cashback => out_trans.cash_backs.push(value),
        };

        self.state.staging_token.clear();

        Ok(())
    }

    fn change_pstate(&mut self, new_state: PriceState) -> ParseResult<()> {
        if PRICE_STATE_REENTRANT_MASK & new_state as u8 == 0
            && self.state.pbitset & new_state as u8 != 0
        {
            parse_fail!(
                self,
                "Pstate {:?} or a state conflicting with it has been entered",
                new_state
            );
        }

        self.state.pstate = new_state;

        self.state.pbitset |= new_state as u8;

        match new_state {
            PriceState::Credit => self.state.pbitset |= PriceState::Reward as u8,
            PriceState::Reward => self.state.pbitset |= PriceState::Credit as u8,
            PriceState::Cashback => self.state.pbitset |= PRICE_STATE_ALL_MASK,
            _ => (),
        }

        Ok(())
    }

    pub fn parse_expr(&mut self, expr_str: impl AsRef<str>) -> ParseResult<CompoundExpr> {
        let expr_str = expr_str.as_ref();

        let mut result = CompoundExpr::default();

        let mut expr = Expr::default();
        let mut trans = ExprTrans::default();
        let mut weak_credit_acc: Option<AccountToken> = None;
        let mut weak_debit_acc: Option<AccountToken> = None;

        self.begin_acc_or_hint(None);

        for (pos, ch) in expr_str.chars().chain("\0".chars()).enumerate() {
            self.state.ch = ch;
            self.state.pos = pos;

            match ch {
                ' ' | '\t' | '\0' => match self.state.estate.clone() {
                    ExprState::Swap => (),
                    ExprState::ExprPartAccAndHint(node) => {
                        if !self.state.staging_token.is_empty() {
                            self.pop_acc_or_hint_token(node, &mut expr)?;
                        }
                    }
                    ExprState::ExprPartPrice => {
                        if !self.state.staging_token.is_empty() {
                            self.pop_price_token(&mut trans)?;
                            expr.trans.push(std::mem::take(&mut trans));
                        }

                        self.state.pbitset = 0;
                        self.change_pstate(PriceState::Debit)?;
                    }
                },
                '\'' | '‘' | '’' => {
                    if let Some(byte_index) = expr_str.char_indices().nth(pos + 1).map(|(i, _)| i) {
                        let comment = expr_str.get(byte_index..);
                        result.comment = Some(comment.unwrap_or_default().to_owned());
                        break;
                    }
                }
                _ => match self.state.estate.clone() {
                    ExprState::Swap => (),
                    ExprState::ExprPartAccAndHint(mut node) => match node.find_mut(ch) {
                        Some(nn) => {
                            self.state.staging_token.push(ch);
                            self.state.estate = ExprState::ExprPartAccAndHint(nn.unsafe_ref());
                        }
                        None => {
                            if !self.state.staging_token.is_empty() {
                                self.pop_acc_or_hint_token(node.clone(), &mut expr)?;
                            }

                            if ch.is_digit(10) || ch == '+' {
                                if expr.accounts.len() <= 1 && weak_credit_acc.is_some() {
                                    expr.accounts.insert(0, weak_credit_acc.clone().unwrap());
                                }
                                if expr.accounts.len() <= 1 && weak_debit_acc.is_some() {
                                    expr.accounts.push(weak_debit_acc.clone().unwrap());
                                }

                                self.state.estate = ExprState::ExprPartPrice;
                                self.state.pbitset = 0;
                                self.change_pstate(PriceState::Debit)?;

                                self.state.staging_token.push(ch);
                            } else {
                                if !self.begin_acc_or_hint(Some(ch)) {
                                    parse_fail!(
                                        self,
                                        "The next account or hint begins with an unknown token"
                                    );
                                }
                            }
                        }
                    },
                    ExprState::ExprPartPrice => match ch {
                        '0'..='9' | '.' => {
                            self.state.staging_token.push(ch);
                        }
                        _ => {
                            if !self.state.staging_token.is_empty() {
                                self.pop_price_token(&mut trans)?;
                            }

                            match ch {
                                '-' => {
                                    self.change_pstate(PriceState::Reward)?;
                                }
                                '@' => {
                                    self.change_pstate(PriceState::Credit)?;
                                }
                                'x' | '*' => {
                                    self.change_pstate(PriceState::Multiple)?;
                                }
                                '/' => {
                                    self.change_pstate(PriceState::Shares)?;
                                }
                                '+' => {
                                    self.change_pstate(PriceState::Cashback)?;
                                }
                                _ => {
                                    if expr.accounts.len() >= 2
                                        && (ch == ',' || ch == '，')
                                    {
                                        weak_credit_acc =
                                            Some(expr.accounts.get(1).unwrap().clone());
                                    } else if expr.accounts.len() >= 2
                                        && (ch == ';' || ch == '；')
                                    {
                                        weak_debit_acc =
                                            Some(expr.accounts.get(1).unwrap().clone());
                                    } else if expr.accounts.len() >= 1
                                    {
                                        weak_credit_acc =
                                            Some(expr.accounts.get(0).unwrap().clone());
                                    }

                                    if !trans.is_empty() {
                                        expr.trans.push(std::mem::take(&mut trans));
                                    }

                                    result.exprs.push(std::mem::take(&mut expr));

                                    if !self.begin_acc_or_hint(
                                        if ch == ',' || ch == '，' || ch == ';' || ch == '；' {
                                            None
                                        } else {
                                            Some(ch)
                                        },
                                    ) {
                                        parse_fail!(
                                            self,
                                            "Sub(Repeated)-expr begins with a unknown token"
                                        );
                                    }
                                }
                            };
                        }
                    },
                },
            }
        }

        if !expr.is_empty() {
            result.exprs.push(expr);
        }

        Ok(result)
    }

    pub fn reset(&mut self) {
        self.state = Default::default();
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
    pub ch: char,
    pub pos: usize,
}

impl std::error::Error for ParseError {}

impl ParseError {
    pub fn new(msg: String, ch: char, pos: usize) -> Self {
        Self { msg, ch, pos }
    }
}

pub type ParseResult<T> = std::result::Result<T, ParseError>;

macro_rules! parse_error {
    ($self:ident, $msg:literal $(,)?) => {
        ParseError::new(String::from($msg), $self.state.ch, $self.state.pos)
    };
    ($self:ident, $fmt:expr, $($arg:tt)*) => {
        ParseError::new(format!($fmt, $($arg)*), $self.state.ch, $self.state.pos)
    };
    ($ch:tt, $pos:tt, $msg:literal $(,)?) => {
        ParseError::new(String::from($msg), $ch, $pos)
    };
    ($fmt:expr, $ch:tt, $pos:tt, $($arg:tt)*) => {
        ParseError::new(format!($fmt, $($arg)*), $ch, $pos)
    };
}

macro_rules! parse_fail {
    ($($tts:tt)*) => {
        return Err(crate::parser::parse_error!($($tts)*))
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ParseError at '{}' ({}): {}",
            self.ch, self.pos, self.msg
        )
    }
}

use hyoubkp_base::AccountToken;
pub(crate) use parse_error;
pub(crate) use parse_fail;
