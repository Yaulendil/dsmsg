//! Generate Messages from the original Dark Souls.

mod data;

use data::{FILL, TEMPLATES};
use rand::prelude::{SliceRandom, ThreadRng};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result},
    ops::Deref,
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
        let fill: Option<Cow<'static, str>> = temp.find('\x1F').map(
            |i| {
                if i == 0 || temp.contains(':') {
                    let mut s: String = FILL.choose(rng)
                        .unwrap().deref().to_string();

                    if !s.is_empty() {
                        //  Replace the first character with its uppercase
                        //      equivalent, in-place.
                        unsafe { s.as_bytes_mut()[0].make_ascii_uppercase(); }
                    }

                    Cow::Owned(s)
                } else {
                    Cow::Borrowed(FILL.choose(rng).unwrap().deref())
                }
            }
        );

        Self { temp, fill }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.fill {
            Some(fill) => {
                let i: usize = self.temp.find('\x1F').unwrap_or_else(|| self.temp.len());
                write!(f, "{}{}{}", &self.temp[..i], &fill, &self.temp[i + 1..])
            }
            _ => self.temp.fmt(f),
        }
    }
}
