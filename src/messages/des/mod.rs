//! des: Generate Messages from the original Demon's Souls.

mod data;

use rand::prelude::{SliceRandom, ThreadRng};

use data::TEMPLATES;


/// Message: A complete Hint Message that could be found in-game. Consists of
///     either one or two `&str`s. One is a Template, and the other, if present,
///     is a Fill phrase.
pub struct Message<'m> {
    temp: &'m str,
    fill: Option<&'m str>,
}

impl Message<'_> {
    /// Create a new `Message`, with at least one randomized `&str`. If the
    ///     chosen string contains a placeholder character, a second `&str` will
    ///     be chosen to fill it.
    pub fn random(rng: &mut ThreadRng) -> Self {
        let (temp, fills): &(&str, Option<&[&str]>) = TEMPLATES.choose(rng).unwrap();
        let fill: Option<&str> = fills.and_then(|array| array.choose(rng).copied());

        Self { temp, fill }
    }
}

impl std::fmt::Display for Message<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.fill {
            Some(fill) if self.temp == "\x1F" => write!(f, "{}", fill),
            Some(fill) => {
                let i: usize = self.temp.find('\x1F').unwrap();
                write!(f, "{}{}{}", &self.temp[..i], &fill, &self.temp[i + 1..])
            }
            _ => write!(f, "{}", &self.temp),
        }
    }
}
