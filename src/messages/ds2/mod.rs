//! Msg2: Generate Messages from Dark Souls II.

mod data;

use rand::prelude::{SliceRandom, ThreadRng};

use crate::util::compound;
use data::{CONJUNCTIONS, TEMPLATES, WORDS, WORDS_MUSINGS};
use super::DsMsg;


/// Segment: A template, and a phrase to be inserted into it. Represents a
///     single complete thought.
struct Segment { main: &'static str, word: &'static str }

impl Segment {
    /// Create a new `Segment`, with a random Template and a random Word.
    pub fn random(rng: &mut ThreadRng) -> Self {
        let main: &str = TEMPLATES.choose(rng).unwrap();
        let word_list: &[&str];

        //  All of the Templates which are allowed to use Musings are four
        //      characters or shorter.
        if main.len() <= 4 {
            word_list = WORDS_MUSINGS;
        } else {
            word_list = &WORDS;
        }

        Self {
            main,
            word: word_list.choose(rng).unwrap(),
        }
    }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i: usize = self.main.find('\x1F').unwrap_or_else(|| self.main.len());
        write!(f, "{}{}{}", &self.main[..i], &self.word, &self.main[i + 1..])
    }
}


/// Message: A complete Hint Message that could be found in-game. Consists of
///     either one or two `Segment`s. If there are two, there will also be a
///     Conjunction `&str`.
pub struct Message {
    p1: Segment,
    p2: Option<(&'static str, Segment)>,
}

impl DsMsg for Message {
    /// Create a new `Message`, with at least one randomized `Segment`. There is
    ///     a chance it will also contain a second `Segment`.
    fn random(rng: &mut ThreadRng) -> Self {
        Self {
            p1: Segment::random(rng),
            p2: if compound(rng) {
                Some((CONJUNCTIONS.choose(rng).unwrap(), Segment::random(rng)))
            } else { None },
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.p2 {
            Some((conj, second)) => write!(f, "{}{}{}", &self.p1, conj, second),
            None => write!(f, "{}", &self.p1),
        }
    }
}
