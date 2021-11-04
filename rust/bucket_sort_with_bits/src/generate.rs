use std::io::{Error, Write};
use std::{env, fs::File};

use bucket_sort_with_bits::utils::random_numbers;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let num = match args[1].parse::<i32>() {
        Ok(i) => i,
        Err(_) => panic!("Please input inger"),
    };

    let numbers = random_numbers(num);

    let path = "numbers.txt";

    let mut output = File::create(path)?;

    for number in numbers {
        writeln!(output, "{}", number)?;
    }

    Ok(())
}
