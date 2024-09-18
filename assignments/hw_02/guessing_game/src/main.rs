fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 42;
    let guesses = [50, 30, 40, 42];
    let mut attempts = 0;
    
    for guess in guesses {
        attempts += 1;

        let result = check_guess(guess, secret);
        if result == 0 {
            println!("{} - Correct!", guess);
            break;
        } else if result == 1 {
            println!("{} - Too high", guess);
        } else if result == -1 {
            println!("{} - Too low", guess);
        }
    }

    println!("You guessed the number in {} attempts.", attempts);
}
