use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess Number Ready 🚀! 🦄️1-100🦄️");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The Secret_Number is {}", secret_number);

    // loop guess more
    loop {
        println!("Please input your number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get nunmber !");

        // number type
        let guess: u32 = match guess.trim().parse() {
            // .expect("please input a number")
            Ok(num) => num,
            Err(_) => {
                println!("please input A f**king number, allright! 😠");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        // compare
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small! ⬇️"),
            Ordering::Greater => println!("Too Big! ⬆️"),
            Ordering::Equal => {
                println!("You Got it finally! 👍 God damn it😌");
                break;
            }
        }
    }
}
