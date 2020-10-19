//! DSMSG: Random generator for Messages that would be found in the Dark Souls
//!     series.

#[cfg(not(any(
    feature = "bloodborne", feature = "demons",
    feature = "ds1", feature = "ds2", feature = "ds3",
)))]
compile_error!("Cannot compile without any Message Sets enabled.");

#[cfg(feature = "ds2")]
#[macro_use]
extern crate lazy_static;
extern crate rand;

#[cfg(feature = "bloodborne")]
mod bb;
#[cfg(feature = "demons")]
mod des;
#[cfg(feature = "ds1")]
mod ds1;
#[cfg(feature = "ds2")]
mod ds2;
#[cfg(feature = "ds3")]
mod ds3;

#[cfg(feature = "bloodborne")]
pub use bb::MessageBB;
#[cfg(feature = "demons")]
pub use des::MessageDeS;
#[cfg(feature = "ds1")]
pub use ds1::MessageDkS1;
#[cfg(feature = "ds2")]
pub use ds2::MessageDkS2;
#[cfg(feature = "ds3")]
pub use ds3::MessageDkS3;

use rand::prelude::{Rng, SliceRandom, thread_rng, ThreadRng};
use std::fmt::Display;


/// Percentage chance for a Message to have multiple clauses, joined by a
///     Conjunction.
const COMPOUND_CHANCE: f64 = 0.5;


/// A Closure that takes an RNG State and returns a dynamic type that
///     implements `DsMsg`.
pub type Generator = fn(&mut ThreadRng) -> Box<dyn DsMsg>;


/// Indicates that a Struct can be used to generate and represent a Message.
pub trait DsMsg: Display {
    fn random(rng: &mut ThreadRng) -> Self
        where Self: Sized;
}


/// A special case of `DsMsg` which may contain a second segment. The two parts
///     will be joined by a Conjunction string.
pub trait DsMulti: Display {
    /// Create a Message with two parts.
    fn double(rng: &mut ThreadRng) -> Self
        where Self: Sized;

    /// Create a Message with one part.
    fn single(rng: &mut ThreadRng) -> Self
        where Self: Sized;
}


impl<M> DsMsg for M
    where M: DsMulti,
{
    /// Create a new Message, with at least one randomized string. There is a
    ///     chance it will also contain a second part, joined to the first by a
    ///     Conjunction.
    fn random(rng: &mut ThreadRng) -> Self {
        if rng.gen_bool(COMPOUND_CHANCE) {
            Self::double(rng)
        } else {
            Self::single(rng)
        }
    }
}


/// A constant slice of small closures that each return a `Box<dyn DsMsg>` of a
///     random generation of their respective messages.
pub const GENERATORS: &[Generator] = &[
    #[cfg(feature = "bloodborne")]
        |r| Box::new(MessageBB::random(r)),
    #[cfg(feature = "demons")]
        |r| Box::new(MessageDeS::random(r)),
    #[cfg(feature = "ds1")]
        |r| Box::new(MessageDkS1::random(r)),
    #[cfg(feature = "ds2")]
        |r| Box::new(MessageDkS2::random(r)),
    #[cfg(feature = "ds3")]
        |r| Box::new(MessageDkS3::random(r)),
];


/// Randomly select from the `GENERATORS` slice, and run it, producing a random
///     message from a random source.
pub fn random_message() -> Box<dyn DsMsg> {
    let mut rng: ThreadRng = thread_rng();

    GENERATORS.choose(&mut rng)
        .expect("DsMsg was not compiled with any Generators!")
        (&mut rng)
}
