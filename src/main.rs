//! DSMSG: Random generator for Messages that would be found in the Dark Souls
//!     series.

extern crate argh;

use argh::{from_env, FromArgs};
use dsmsg::*;
use rand::prelude::{IteratorRandom, SliceRandom, thread_rng, ThreadRng};


/// Error message for when there are no Generators available. This should NEVER
///     happen, because compile-time checks ensure that at least one Generator
///     will be active.
const NO_GENERATOR: &str = "Failed to select a Message Generator; DsMsg may \
need to be recompiled.";


/**
Randomly generate Messages that could be found in the Dark Souls series.

Messages from a specific game may be specified by command line options. If
multiple games are specified, one of them will be randomly selected to be
generated. If no options are specified, it is interpreted the same as if ALL
have been selected.
*/ //  NOTE: Block comment is necessary here to properly lay out help text.
#[derive(FromArgs)]
struct CommandOpts {
    /// generate a message from Bloodborne
    #[cfg(feature = "bloodborne")]
    #[argh(switch)]
    bb: bool,

    /// generate a message from Demon's Souls
    #[cfg(feature = "demons")]
    #[argh(switch)]
    des: bool,

    /// generate a message from Dark Souls I
    #[cfg(feature = "ds1")]
    #[argh(switch)]
    ds1: bool,

    /// generate a message from Dark Souls II
    #[cfg(feature = "ds2")]
    #[argh(switch)]
    ds2: bool,

    /// generate a message from Dark Souls III
    #[cfg(feature = "ds3")]
    #[argh(switch)]
    ds3: bool,

    /// generate a message from Sekiro
    #[cfg(feature = "sekiro")]
    #[argh(switch)]
    sek: bool,
}


impl From<CommandOpts> for Vec<bool> {
    fn from(opt: CommandOpts) -> Self {
        vec![
            #[cfg(feature = "bloodborne")] opt.bb,
            #[cfg(feature = "demons")] opt.des,
            #[cfg(feature = "ds1")] opt.ds1,
            #[cfg(feature = "ds2")] opt.ds2,
            #[cfg(feature = "ds3")] opt.ds3,
            #[cfg(feature = "sekiro")] opt.sek,
        ]
    }
}


fn main() -> Result<(), &'static str> {
    let mut rng: ThreadRng = thread_rng();
    match {
        let opts: Vec<bool> = Vec::from(from_env::<CommandOpts>());

        if opts.iter().any(|&b| b) {
            let mut filter = opts.iter().copied();
            GENERATORS.iter()
                .filter(move |_| filter.next().unwrap_or_default())
                .choose(&mut rng)
        } else {
            GENERATORS.choose(&mut rng)
        }
    } {
        Some(generate) => {
            println!("{}", generate(&mut rng));
            Ok(())
        }
        None => Err(NO_GENERATOR),
    }
}
