use std::env;

fn fizz_buzz(num: i32) -> String {
    match (num % 3, num % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("fizz"),
        (_, 0) => String::from("Buzz"),
        _ => num.to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let num = match args[1].parse::<i32>() {
        Ok(i) => i,
        Err(_) => panic!("Please input inger"),
    };

    for i in 1..=num {
        println!("{}", fizz_buzz(i));
    }
}
