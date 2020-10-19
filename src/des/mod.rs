//! Generate Messages from the original Demon's Souls.

mod data;

use data::TEMPLATES;
use rand::prelude::{SliceRandom, ThreadRng};
use std::fmt::{Display, Formatter, Result};
use super::DsMsg;


/// A complete Hint Message that could be found in Demon's Souls. Consists of
///     either one or two strings. One is a Template, and the other, if present,
///     is a Fill phrase.
pub struct MessageDeS {
    temp: &'static str,
    fill: Option<&'static str>,
}

impl DsMsg for MessageDeS {
    /// Create a new `MessageDeS`, with at least one randomized string. If the
    ///     chosen string contains a placeholder character, a second string will
    ///     be chosen to fill it.
    fn random(rng: &mut ThreadRng) -> Self {
        let (temp, fills): &(&str, Option<&[&str]>) = TEMPLATES.choose(rng).unwrap();
        let fill: Option<&str> = fills.and_then(|array| array.choose(rng).copied());

        Self { temp, fill }
    }
}

impl Display for MessageDeS {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.fill {
            Some(fill) if self.temp == "\x1F" => fill.fmt(f),
            Some(fill) => match self.temp.find('\x1F') {
                Some(i) => write!(
                    f, "{}{}{}",
                    &self.temp[..i], &fill, &self.temp[i + 1..],
                ),
                None => self.temp.fmt(f),
            }
            None => self.temp.fmt(f),
        }
    }
}
