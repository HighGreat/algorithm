use std::env;
use std::fs::File;
use std::io::Write;

use chrono::prelude::Utc;

use bucket_sort_with_bits::bucket_sort;
use bucket_sort_with_bits::utils::{build_numbers, read_lines};

fn main() {
    println!("{} start", Utc::now());
    let args: Vec<String> = env::args().collect();

    let filename = match args[1].parse::<String>() {
        Ok(i) => i,
        Err(_) => panic!("something goes wrong."),
    };

    println!("{} loading file...", Utc::now());

    let lines = match read_lines(filename) {
        Ok(i) => i,
        Err(_) => panic!("something goes wrong."),
    };

    let mut numbers = build_numbers(lines);

    println!("{} sorting...", Utc::now());

    bucket_sort::sort(&mut numbers);

    println!("{} writting result...", Utc::now());

    let mut file = match File::create("result.txt") {
        Ok(i) => i,
        Err(_) => panic!("something goes wrong"),
    };

    for number in numbers {
        writeln!(file, "{}", number).unwrap();
    }

    println!("{} finished", Utc::now());
}
