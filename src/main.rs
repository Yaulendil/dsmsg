#[macro_use]
extern crate lazy_static;
extern crate rand;

mod msg1;
mod msg2;
mod msg3;
#[cfg(test)]
mod test;
mod util;


use rand::{Rng, thread_rng};


fn main() {
    let mut rng = thread_rng();

    match rng.gen_range(1, 4) {
        1 => println!("{}", msg1::Message::random(&mut rng)),
        2 => println!("{}", msg2::Message::random(&mut rng)),
        3 => println!("{}", msg3::Message::random(&mut rng)),
        _ => unreachable!(),
    }
}
