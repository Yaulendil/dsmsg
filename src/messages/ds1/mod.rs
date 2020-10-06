//! Msg1: Generate Messages from the original Dark Souls.

mod data;

use crate::util::capitalize;
use data::{FILL, TEMPLATES};
use rand::prelude::{SliceRandom, ThreadRng};
use std::{
    borrow::Cow,
    ops::Deref,
};
use super::DsMsg;


/// Message: A complete Hint Message that could be found in-game. Consists of
///     either one or two `&str`s. One is a Template, and the other, if present,
///     is a Fill phrase.
pub struct Message {
    temp: &'static str,
    fill: Option<Cow<'static, str>>,
}

impl DsMsg for Message {
    /// Create a new `Message`, with at least one randomized `&str`. If the
    ///     chosen string contains a placeholder character, a second string will
    ///     be chosen to fill it.
    fn random(rng: &mut ThreadRng) -> Self {
        let temp: &'static str = TEMPLATES.choose(rng).unwrap();
        let fill: Option<Cow<'static, str>> = temp.find('\x1F').map(
            |i| {
                if i == 0 || temp.contains(':') {
                    let mut s: String = FILL.choose(rng)
                        .unwrap().deref().to_string();

                    capitalize(&mut s);
                    Cow::Owned(s)
                } else {
                    Cow::Borrowed(FILL.choose(rng).unwrap().deref())
                }
            }
        );

        Self { temp, fill }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.fill {
            Some(fill) => {
                let i: usize = self.temp.find('\x1F').unwrap_or_else(|| self.temp.len());
                write!(f, "{}{}{}", &self.temp[..i], &fill, &self.temp[i + 1..])
            }
            _ => write!(f, "{}", &self.temp),
        }
    }
}
