//! Generate Messages from the original Dark Souls.

mod data;

use data::{FILL, TEMPLATES};
use rand::prelude::{SliceRandom, ThreadRng};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result},
};
use super::DsMsg;


/// A complete Hint Message that could be found in Dark Souls I. Consists of
///     either one or two strings. One is a Template, and the other, if present,
///     is a Fill phrase.
pub struct Message {
    temp: &'static str,
    fill: Option<Cow<'static, str>>,
}

impl DsMsg for Message {
    /// Create a new `Message`, with at least one randomized string. If the
    ///     chosen string contains a placeholder character, a second string will
    ///     be chosen to fill it.
    fn random(rng: &mut ThreadRng) -> Self {
        let temp: &'static str = TEMPLATES.choose(rng).unwrap();
        let fill: Option<Cow<'static, str>> = match temp.find('\x1F')
        {
            Some(i) if i == 0 || temp.contains(':') => {
                let mut s: String = FILL.choose(rng).unwrap().to_string();

                //  Replace the first character with its uppercase equivalent.
                unsafe { s.as_bytes_mut().first_mut().map(u8::make_ascii_uppercase); }

                Some(Cow::Owned(s))
            }
            Some(_) => Some(Cow::Borrowed(*FILL.choose(rng).unwrap())),
            None => None,
        };

        Self { temp, fill }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.fill {
            Some(fill) if self.temp == "\x1F" => fill.fmt(f),
            Some(fill) => {
                let i: usize = self.temp.find('\x1F').unwrap();
                write!(f, "{}{}{}", &self.temp[..i], &fill, &self.temp[i + 1..])
            }
            _ => self.temp.fmt(f),
        }
    }
}
