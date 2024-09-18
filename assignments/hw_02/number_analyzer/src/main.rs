fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [12, 5, 8452121, 150, 33, 7, 19, 1, 22243, 5];

    for number in numbers {
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else if is_even(number) {
            println!("Even");
        } else {
            println!("Odd");
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum: {}", sum);

    let mut largest = numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest: {}", largest);
}