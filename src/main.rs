//! DSMSG: Random generator for Messages that would be found in the Dark Souls
//!     series.

extern crate argh;

use argh::{from_env, FromArgs};
use dsmsg::*;
use rand::prelude::{SliceRandom, thread_rng, ThreadRng};


const NO_GENERATOR: &str = (
    "Failed to select a Message Generator; DsMsg may need to be recompiled."
);


/**
Randomly generate Messages that could be found in the Dark Souls series.

Messages from a specific game may be specified by command line options. If
multiple games are specified, one of them will be randomly selected to be
generated. If no options are specified, it is interpreted the same as if ALL
have been selected.
**/ //  NOTE: Block comment is necessary here to properly lay out help text.
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
}


impl CommandOpts {
    const fn any(&self) -> bool {
        let mut any: bool = false;

        #[cfg(feature = "bloodborne")] { any |= self.bb; }
        #[cfg(feature = "demons")] { any |= self.des; }
        #[cfg(feature = "ds1")] { any |= self.ds1; }
        #[cfg(feature = "ds2")] { any |= self.ds2; }
        #[cfg(feature = "ds3")] { any |= self.ds3; }

        any
    }
}


fn main() -> Result<(), &'static str> {
    let opt: CommandOpts = from_env();
    let vec_gen: Vec<Generator> = {
        if opt.any() {
            let mut vec: Vec<Generator> = Vec::with_capacity(5);

            #[cfg(feature = "bloodborne")] {
                if opt.bb { vec.push(|r| Box::new(MessageBB::random(r))); }
            }
            #[cfg(feature = "demons")] {
                if opt.des { vec.push(|r| Box::new(MessageDeS::random(r))); }
            }
            #[cfg(feature = "ds1")] {
                if opt.ds1 { vec.push(|r| Box::new(MessageDkS1::random(r))); }
            }
            #[cfg(feature = "ds2")] {
                if opt.ds2 { vec.push(|r| Box::new(MessageDkS2::random(r))); }
            }
            #[cfg(feature = "ds3")] {
                if opt.ds3 { vec.push(|r| Box::new(MessageDkS3::random(r))); }
            }

            vec
        } else {
            Vec::from(GENERATORS)
        }
    };

    let mut rng: ThreadRng = thread_rng();
    match vec_gen.choose(&mut rng) {
        Some(generate) => {
            println!("{}", generate(&mut rng));
            Ok(())
        }
        None => Err(NO_GENERATOR),
    }
}
