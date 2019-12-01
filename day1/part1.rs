use std::{
    io::{BufReader, prelude::*},
    fs::File,
};

fn main() {
    let input = File::open("input").unwrap();
    let input = BufReader::new(input);
    let modules = input.lines().filter_map(|module| module.unwrap().parse::<i32>().ok());
    let fuels = modules.map(|module| module / 3 - 2);
    println!("{}", fuels.sum::<i32>());
}
