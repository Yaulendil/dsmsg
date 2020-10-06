#[cfg(not(any(
    feature = "bloodborne", feature = "demons",
    feature = "ds1", feature = "ds2", feature = "ds3",
)))]
compile_error!("Cannot compile without any Message Sets enabled.");

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
pub use bb::Message as MessageBB;
#[cfg(feature = "demons")]
pub use des::Message as MessageDeS;
#[cfg(feature = "ds1")]
pub use ds1::Message as MessageDkS1;
#[cfg(feature = "ds2")]
pub use ds2::Message as MessageDkS2;
#[cfg(feature = "ds3")]
pub use ds3::Message as MessageDkS3;

use rand::prelude::{SliceRandom, thread_rng, ThreadRng};


/// Indicates that a Struct can be used to generate and represent a Message.
pub trait DsMsg: std::fmt::Display {
    fn random(rng: &mut ThreadRng) -> Self
        where Self: Sized;
}


/// A special case of `DsMsg` which may contain a second segment. The two parts
///     will be joined by a Conjunction string.
pub trait DsMulti: DsMsg {
    /// Create a `Message` with two parts.
    fn double(rng: &mut ThreadRng) -> Self
        where Self: Sized;

    /// Create a `Message` with one part.
    fn single(rng: &mut ThreadRng) -> Self
        where Self: Sized;
}


/// A constant slice of small closures that each return a `Box<dyn DsMsg>` of a
///     random generation of their respective messages.
pub const GENERATORS: &[fn(&mut ThreadRng) -> Box<dyn DsMsg>] = &[
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
