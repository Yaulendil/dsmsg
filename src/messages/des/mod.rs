//! Generate Messages from the original Demon's Souls.

mod data;

use data::TEMPLATES;
use rand::prelude::{SliceRandom, ThreadRng};
use std::fmt::{Display, Formatter, Result};
use super::DsMsg;


/// A complete Hint Message that could be found in Demon's Souls. Consists of
///     either one or two strings. One is a Template, and the other, if present,
///     is a Fill phrase.
pub struct Message {
    temp: &'static str,
    fill: Option<&'static str>,
}

impl DsMsg for Message {
    /// Create a new `Message`, with at least one randomized string. If the
    ///     chosen string contains a placeholder character, a second string will
    ///     be chosen to fill it.
    fn random(rng: &mut ThreadRng) -> Self {
        let (temp, fills): &(&str, Option<&[&str]>) = TEMPLATES.choose(rng).unwrap();
        let fill: Option<&str> = fills.and_then(|array| array.choose(rng).copied());

        Self { temp, fill }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.fill {
            Some(fill) if self.temp == "\x1F" => write!(f, "{}", fill),
            Some(fill) => {
                let i: usize = self.temp.find('\x1F').unwrap();
                write!(f, "{}{}{}", &self.temp[..i], &fill, &self.temp[i + 1..])
            }
            _ => self.temp.fmt(f),
        }
    }
}
