//! DSMSG: Random generator for Messages that would be found in the Dark Souls
//!     series.

#[cfg(feature = "ds2")]
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod messages;
#[cfg(test)]
mod test;
mod util;

pub use messages::*;
