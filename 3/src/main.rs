use std::io::prelude::*;
use std::{env, io};

use crate::lib::{visit_count, visit_robot_count};

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let stdin = io::stdin();
    let contents = stdin.lock().lines().next().unwrap().unwrap();

    println!("Visted Houses: {}", visit_count(contents.clone()));
    println!("Visted Houses with robot: {}", visit_robot_count(contents));
}
