use super::*;


#[test]
fn test_main() {
    for i in 1..6 {
        print!("Main-{}: ", i);
        main();
    }
}


#[test]
fn test_ds1() {
    for i in 1..301 {
        println!("1-{:0>4}: {}", i, msg1::Message::random(&mut thread_rng()));
    }
}


#[test]
fn test_ds2() {
    for i in 1..301 {
        println!("2-{:0>4}: {}", i, msg2::Message::random(&mut thread_rng()));
    }
}


#[test]
fn test_ds3() {
    for i in 1..301 {
        println!("3-{:0>4}: {}", i, msg3::Message::random(&mut thread_rng()));
    }
}
