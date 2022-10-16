use std::io::prelude::*;
use std::{env, io, str};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let stdin = io::stdin();
    let secret = stdin.lock().lines().next().unwrap().unwrap();

    let mut i = 1;

    loop {
        let to_hash = secret.clone() + &i.to_string();
        let hash: String = format!("{:x}", &md5::compute(&to_hash));
        println!("input: {} -> Hash {}", to_hash.clone(), hash);
        if hash.starts_with("000000") {
            break;
        }
        i += 1;
    }
}
