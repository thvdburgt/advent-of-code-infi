use std::fs::File;
use std::io::prelude::*;

extern crate advent_of_code_infi as advent;

fn main() {
    let mut file = File::open("input").expect("Could not open input");
    let mut input = String::new();
    file.read_to_string(&mut input).expect(
        "Could not read input",
    );

    let input = input.trim();
    let answer1 = advent::puzzle1(input);
    println!("the answer is {}", answer1);
}
