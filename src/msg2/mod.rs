//! Msg2: Generate Messages from Dark Souls II.

mod data;

use rand::prelude::ThreadRng;

use crate::util::{choose, compound};
use data::{CONJUNCTIONS, WORDS, WORDS_MUSINGS, TEMPLATES};


/// Segment: A template, and a phrase to be inserted into it. Represents a
///     single complete thought.
struct Segment<'s> { main: &'s str, word: &'s str, }

impl Segment<'_> {
    /// Create a new `Segment`, with a random Template and a random Word.
    pub fn random(rng: &mut ThreadRng) -> Self {
        let main = choose(TEMPLATES, rng);
        let word_list;

        //  All of the Templates which are allowed to use Musings are four
        //      characters or shorter.
        if main.len() <= 4 {
            word_list = WORDS_MUSINGS;
        } else {
            word_list = &WORDS;
        }

        Self {
            main,
            word: choose(word_list, rng),
        }
    }
}

impl std::fmt::Display for Segment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i: usize = self.main.find('\x1F').unwrap_or_else(|| self.main.len());
        write!(f, "{}{}{}", &self.main[..i], &self.word, &self.main[i + 1..])
    }
}


/// Message: A complete Hint Message that could be found in-game. Consists of
///     either one or two `Segment`s. If there are two, there will also be a
///     Conjunction `&str`.
pub struct Message<'m> {
    p1: Segment<'m>,
    p2: Option<(&'m str, Segment<'m>)>,
}

impl Message<'_> {
    /// Create a new `Message`, with at least one randomized `Segment`. There is
    ///     a chance it will also contain a second `Segment`.
    pub fn random(rng: &mut ThreadRng) -> Self {
        Self {
            p1: Segment::random(rng),
            p2: {
                if compound(rng) {
                    Some((choose(CONJUNCTIONS, rng), Segment::random(rng)))
                } else { None }
            },
        }
    }
}

impl std::fmt::Display for Message<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((conj, second)) = &self.p2 {
            write!(f, "{}{}{}", &self.p1, conj, second)
        } else {
            write!(f, "{}", &self.p1)
        }
    }
}
