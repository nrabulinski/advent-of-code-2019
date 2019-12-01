use std::{
    io::{BufReader, prelude::*},
    fs::File,
};

fn calc_fuel(f: i32) -> i32 {
    let f = f / 3 - 2;
    if f > 0 { f + calc_fuel(f) } else { 0 }
}

fn main() {
    let input = File::open("input").unwrap();
    let input = BufReader::new(input);
    let modules = input.lines().filter_map(|module| module.unwrap().parse::<i32>().ok());
    let fuels = modules.map(calc_fuel);
    println!("{}", fuels.sum::<i32>());
}
