//pub type Price = i32;

use std::{fmt::Display, str::FromStr};

use crate::error::{bail, Error};

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Price(i32);

impl FromStr for Price {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let integer_part = parts.next();
        let fractional_part = parts.next();

        if integer_part.is_none() {
            return Ok(Self(0));
        }

        let integer_part = integer_part.unwrap().parse::<i32>()?;

        if fractional_part.is_none() {
            return Ok(Self::new_unchecked(integer_part, 0));
        }

        let fractional_part = fractional_part.unwrap();
        
        let fractional_part = if fractional_part.len() == 2 {
            fractional_part.parse::<i32>()?
        } else if fractional_part.len() == 1 {
            fractional_part.parse::<i32>()? * 10
        } else {
            bail!("Fractional part must less than 100, but current is '{}'", fractional_part);
        };

        if fractional_part >= 100 {
        }

        if parts.next().is_some() {
            bail!("Unexcept part after fractional part");
        }

        Ok(Self::new_unchecked(integer_part, fractional_part))
    }
}

impl std::ops::Sub for Price {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:02}", self.integer_part(), self.fractional_part())
    }
}

impl Price {
    pub fn as_raw(&self) -> i32 {
        self.0
    }

    pub fn integer_part(&self) -> i32 {
        self.0 / 100
    }

    pub fn fractional_part(&self) -> i32 {
        (self.0 % 100).abs()
    }

    pub fn new_unchecked(integer_part: i32, fractional_part: i32) -> Self {
        Self(integer_part * 100 + fractional_part)
    }
}
