use std::io::stdin;
use rand::random;

pub fn main() {
    let random_no: u8 = random();
    for _ in 0..5 {
        let mut input_str: String = Default::default();
        stdin().read_line(&mut input_str).expect("Enter a number!");
        // println!("input no is: {}", input_str.to_string());
        // let input_no: u8 = input_str.trim().parse().unwrap();
        let input_no: u8 = match input_str.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error: Invalid input or number > 255!");
                break;
            }
        };

        if input_no < random_no {
            println!("Too small!");
        } else if input_no > random_no {
            println!("Too big!");
        } else {
            println!("You win!");
        }
    }
    println!("actual random no is:: {}", random_no);
}