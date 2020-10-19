//! Generate Messages from Dark Souls II.

mod data;

use data::{CONJUNCTIONS, TEMPLATES, WORDS, WORDS_MUSINGS};
use rand::prelude::{SliceRandom, ThreadRng};
use std::fmt::{Display, Formatter, Result};
use super::DsMulti;


/// A template, combined with a phrase to be inserted into it. Represents a
///     single complete thought.
struct Segment { main: &'static str, word: &'static str }

impl Segment {
    /// Create a new `Segment`, with a random Template and a random Word.
    pub fn random(rng: &mut ThreadRng) -> Self {
        let main: &str = TEMPLATES.choose(rng).unwrap();

        //  All of the Templates which are allowed to use Musings are four
        //      characters or shorter.
        let word_list: &[&str] = if main.len() <= 4 {
            WORDS_MUSINGS
        } else {
            &WORDS
        };

        Self {
            main,
            word: word_list.choose(rng).unwrap(),
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.main == "\x1F" {
            self.word.fmt(f)
        } else {
            match self.main.find('\x1F') {
                Some(i) => write!(
                    f, "{}{}{}",
                    &self.main[..i], &self.word, &self.main[i + 1..],
                ),
                None => self.main.fmt(f),
            }
        }
    }
}


/// A complete Hint Message that could be found in Dark Souls II. Consists of
///     either one or two parts. If there are two, there will also be a
///     Conjunction string to join them.
pub struct MessageDkS2 {
    p1: Segment,
    p2: Option<(&'static str, Segment)>,
}

impl DsMulti for MessageDkS2 {
    /// Create a `MessageDkS2` with two parts.
    fn double(rng: &mut ThreadRng) -> Self {
        Self {
            p1: Segment::random(rng),
            p2: Some((CONJUNCTIONS.choose(rng).unwrap(), Segment::random(rng))),
        }
    }

    /// Create a `MessageDkS2` with one part.
    fn single(rng: &mut ThreadRng) -> Self {
        Self {
            p1: Segment::random(rng),
            p2: None,
        }
    }
}

impl Display for MessageDkS2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.p2 {
            Some((conj, second)) => write!(f, "{}{}{}", &self.p1, conj, second),
            None => self.p1.fmt(f),
        }
    }
}
