use std::io::prelude::*;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let stdin = io::stdin();
    let secret = stdin.lock().lines().next().unwrap().unwrap();

    println!("Hash {:x}", md5::compute(b"asdf"));
}

