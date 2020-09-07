//! Msg1: Generate Messages from the original Dark Souls.

mod data;

use rand::prelude::ThreadRng;

use crate::util::{capitalize, choose};
use data::{FILL, TEMPLATES};


/// Message: A complete Hint Message that could be found in-game. Consists of
///     either one or two `&str`s. One is a Template, and the other, if present,
///     is a Fill phrase.
pub struct Message<'m> {
    temp: &'m str,
    fill: Option<String>,
}

impl Message<'_> {
    /// Create a new `Message`, with at least one randomized `&str`. If the
    ///     chosen string contains a placeholder character, a second `&str` will
    ///     be chosen to fill it.
    pub fn random(rng: &mut ThreadRng) -> Self {
        let temp = choose(TEMPLATES, rng);
        let fill;

        if temp.contains('\x1F') {
            let mut s = choose(FILL, rng).to_owned();

            if temp.starts_with('\x1F') || temp.contains(':') {
                capitalize(&mut s);
            }

            fill = Some(s);
        } else { fill = None; }

        Self {
            temp,
            fill,
        }
    }
}

impl std::fmt::Display for Message<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(fill) = &self.fill {
            let i: usize = self.temp.find('\x1F').unwrap_or_else(|| self.temp.len());
            write!(f, "{}{}{}", &self.temp[..i], &fill, &self.temp[i + 1..])
        } else {
            write!(f, "{}", &self.temp)
        }
    }
}