extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数当てゲーム");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Filed to read");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed: {} ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さい"),
            Ordering::Greater => println!("大きい"),
            Ordering::Equal => {
                println!("You Win");
                break;
            },
        }
    }

}
