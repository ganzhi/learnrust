use std::io;
use std::cmp::Ordering;
use std::fs::File;
use rand::Rng;

mod model;
use crate::model::User;

mod guess;
use guess::Guess;

fn main1() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let some_u8_value = Some(0u8);
    if some_u8_value == Some(0u8) {
        println!("Hello, 0u8");
    }

    let user1 = User{
        name: String::from("Oliver Twist"),
        email: String::from("Hello, world"),
        active: true,
        sign_in_count: 1,        
    };

    println!("Your user name is {}", user1.name);

    let f = File::open("hello.txt");
    use std::io::ErrorKind;
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let g1 = Guess::new(guess);

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn main(){
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);    
}