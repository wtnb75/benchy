use test_env_log::test;

use super::*;
use rand::Rng;
use std::{thread, time};

#[test]
fn empty5sec() {
    for _ in Benchy::new_duration("empty5sec", Duration::from_secs(5)) {
        // empty
    }
    // finished
}

#[test]
fn rng5sec() {
    let mut rng = rand::thread_rng();
    for _ in Benchy::new_duration("rng5sec", Duration::from_secs(5)) {
        let _: i32 = rng.gen();
    }
}

#[test]
fn rng5cnt() {
    let mut rng = rand::thread_rng();
    let mut cnt = 0;
    for i in Benchy::new_n("rng5cnt", 5) {
        println!("cur={}", i);
        let _: i32 = rng.gen();
        cnt += 1;
    }
    println!("cnt={}", cnt);
}

#[test]
fn data1024() {
    let mut d = Data::new();
    for i in 0..1024 {
        d.register_val(i);
    }
    println!("result: {:?}", d);
    println!("100-th: {}", d.nth_from_low(100).unwrap());
    println!("300-th: {}", d.nth_from_low(300).unwrap());
    println!("high 300-th: {}", d.nth_from_high(300).unwrap());
    println!("20%-th: {}", d.percentile(20.0).unwrap());
    println!("95%-th: {}", d.percentile(95.0).unwrap());
    println!("99%-th: {}", d.percentile(99.0).unwrap());
}

#[test]
fn data0() {
    let d = Data::new();
    assert_eq!("(initialized)", format!("{}", d));
}

#[test]
fn data1() {
    let mut b = Benchy::new_n("first", 5);
    b.next();
    assert_eq!("(no data)", format!("{}", b.data));
}

#[test]
fn data2() {
    let mut b = Benchy::new_n("second", 5);
    b.next();
    b.data.register_val(1);
    assert_eq!(
        "cnt=1, avg=1.000, median=0, mode=1, stdev=0.000, min/max=1/1",
        format!("{}", b.data)
    );
}

#[test]
fn datax() {
    let mut rng = rand::thread_rng();
    let mut d = Data::new();
    d.cnt += 1;
    for _ in 0..50000 {
        d.register_val(rng.gen_range(10, 1024) + rng.gen_range(10, 1024));
    }
    println!("{:?}\n{}", d, d);
}

fn get_fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => get_fib(n - 2) + get_fib(n - 1),
    }
}

#[test]
fn fib() {
    for _ in Benchy::new_duration("fib3sec", Duration::from_secs(3)) {
        get_fib(40);
    }
}

#[test]
fn sleep() {
    let mut rng = rand::thread_rng();
    for _ in Benchy::new_duration("sleep", Duration::from_secs(5)) {
        thread::sleep(time::Duration::from_millis(rng.gen_range(10, 100)));
    }
}
