use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rand::prelude::SliceRandom;

pub fn random_numbers(size: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();

    let mut numbers: Vec<i32> = (0..=size).map(|n| n).collect();

    numbers.shuffle(&mut rng);

    numbers
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn build_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
    lines
        .map(|line| {
            let string = match line {
                Ok(n) => n,
                Err(_) => panic!("input number!"),
            };

            let number = match string.parse::<i32>() {
                Ok(i) => i,
                Err(_) => panic!("value must be integer"),
            };

            number
        })
        .collect()
}
