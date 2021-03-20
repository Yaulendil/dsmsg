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
pub struct MessageDkS1 {
    temp: &'static str,
    fill: Option<Cow<'static, str>>,
}

impl DsMsg for MessageDkS1 {
    /// Create a new `MessageDkS1`, with at least one randomized string. If the
    ///     chosen string contains a placeholder character, a second string will
    ///     be chosen to fill it.
    fn random(rng: &mut ThreadRng) -> Self {
        let temp: &'static str = TEMPLATES.choose(rng).unwrap();
        let fill: Option<Cow<'static, str>> = match temp.find('\x1F')
        {
            Some(i) if i == 0 || temp.contains(':') => {
                let mut s: String = FILL.choose(rng).unwrap().to_string();

                //  Replace the first character with its uppercase equivalent.
                unsafe {
                    //  SAFETY: This should be safe because only one byte is
                    //      being affected, and it is only affected if it is not
                    //      part of a multi-byte UTF-8 sequence.
                    s.as_bytes_mut().first_mut().map(u8::make_ascii_uppercase);
                }

                Some(Cow::Owned(s))
            }
            Some(_) => Some(Cow::Borrowed(*FILL.choose(rng).unwrap())),
            None => None,
        };

        Self { temp, fill }
    }
}

impl Display for MessageDkS1 {
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
