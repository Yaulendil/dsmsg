//! Msg1: Generate Messages from the original Dark Souls.

mod data;

use rand::prelude::{SliceRandom, ThreadRng};

use crate::util::capitalize;
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
        let temp: &str = TEMPLATES.choose(rng).unwrap();
        let fill: Option<String> = match temp.find('\x1F') {
            Some(i) => {
                let mut s = String::from(*FILL.choose(rng).unwrap());

                if i == 0 || temp.contains(':') { capitalize(&mut s); }

                Some(s)
            }
            None => { None }
        };

        Self { temp, fill }
    }
}

impl std::fmt::Display for Message<'_> {
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
