//! DSMSG: Random generator for Messages that would be found in the Dark Souls
//!     series.

#[cfg(not(any(
    feature = "bloodborne", feature = "demons",
    feature = "ds1", feature = "ds2", feature = "ds3",
    feature = "eldenring",
    feature = "sekiro",
)))]
compile_error!("Cannot compile without any Message Sets enabled. Enable \
at least one of the following Features:\
\n- bloodborne\n- demons\n- ds1\n- ds2\n- ds3\n- eldenring\n- sekiro");

#[cfg(feature = "ds2")]
#[macro_use]
extern crate lazy_static;
extern crate rand;
#[macro_use]
extern crate static_assertions;

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
#[cfg(feature = "sekiro")]
mod sek;
#[cfg(feature = "eldenring")]
mod er1;

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
#[cfg(feature = "sekiro")]
pub use sek::MessageSek;
#[cfg(feature = "eldenring")]
pub use er1::MessageEr1;

use rand::prelude::{Rng, SliceRandom, thread_rng, ThreadRng};
use std::fmt::Display;


/// Percentage chance for a Message to have multiple clauses, joined by a
///     Conjunction.
const COMPOUND_CHANCE: f64 = 0.5;


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


impl<M: DsMulti> DsMsg for M {
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


/// A Closure that takes an RNG State and returns a dynamic type that
///     implements `DsMsg`.
pub type Generator = fn(&mut ThreadRng) -> Box<dyn DsMsg>;


/// A constant slice of small closures that each return a `Box<dyn DsMsg>` of a
///     random generation of their respective messages.
pub const GENERATORS: &[fn(&mut ThreadRng) -> Box<dyn DsMsg>] = &[
    #[cfg(feature = "bloodborne")]  |r| Box::new(MessageBB::random(r)),
    #[cfg(feature = "demons")]      |r| Box::new(MessageDeS::random(r)),
    #[cfg(feature = "ds1")]         |r| Box::new(MessageDkS1::random(r)),
    #[cfg(feature = "ds2")]         |r| Box::new(MessageDkS2::random(r)),
    #[cfg(feature = "ds3")]         |r| Box::new(MessageDkS3::random(r)),
    #[cfg(feature = "eldenring")]   |r| Box::new(MessageEr1::random(r)),
    #[cfg(feature = "sekiro")]      |r| Box::new(MessageSek::random(r)),
];


//  Double check to make ***absolutely certain*** that this cannot be empty.
const_assert_ne!(GENERATORS.len(), 0);


/// Randomly select from the `GENERATORS` slice, and run it, producing a random
///     message from a random source.
pub fn random_message() -> Box<dyn DsMsg> {
    let mut rng: ThreadRng = thread_rng();

    //  SAFETY: This should be safe because `GENERATORS` is ensured to not be
    //      empty by compile-time checks.
    GENERATORS.choose(&mut rng)
        .unwrap_or_else(|| unsafe { std::hint::unreachable_unchecked() })
        (&mut rng)
}
