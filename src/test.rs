use super::*;


const ITERS: usize = 300;
const DIGIT: usize = 3;


#[test]
fn test_main() {
    for i in 1..6 {
        print!("Main-{}: ", i);
        main();
    }
}


#[test]
fn test_ds1() {
    for i in 1..=ITERS {
        println!(
            "1-{i:0d$}: {txt}",
            i=i, d=DIGIT, txt=MessageDS1::random(&mut thread_rng()),
        );
    }
}


#[test]
fn test_ds2() {
    for i in 1..=ITERS {
        println!(
            "2-{i:0d$}: {txt}",
            i=i, d=DIGIT, txt=MessageDS2::random(&mut thread_rng()),
        );
    }
}


#[test]
fn test_ds3() {
    for i in 1..=ITERS {
        println!(
            "3-{i:0d$}: {txt}",
            i=i, d=DIGIT, txt=MessageDS3::random(&mut thread_rng()),
        );
    }
}


#[cfg(feature="bloodborne")]
#[test]
fn test_bb() {
    for i in 1..=ITERS {
        println!(
            "B-{i:0d$}: {txt}",
            i=i, d=DIGIT, txt=MessageBlood::random(&mut thread_rng()),
        );
    }
}
