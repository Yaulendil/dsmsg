extern crate rand;

mod msg1;
mod msg3;
#[cfg(test)]
mod test;
mod util;


use rand::{Rng, thread_rng};


fn main() {
    let mut rng = util::thread_rng();

    match rng.gen_range(1, 4) {
        1 => println!("{}", msg1::Message::random(&mut rng)),
        2 => unimplemented!(),
        3 => println!("{}", msg3::Message::random(&mut rng)),
        _ => unreachable!(),
    }
}
