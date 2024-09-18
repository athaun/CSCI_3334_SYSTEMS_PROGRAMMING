
fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let old_x = x;
    let mut x = x;
    let mut num = 0;

    while x > 0 {
        num = (num * 10) + (x % 10);
        
        x = x / 10;
    }

    return num == old_x;
}

fn main () {
    print!("{}\n", is_palindrome(1011));
}

// fn most_frequent_word(text: &str) -> (String, usize) {
//     let words = text.split_whitespace();
//     let mut max_word : String;
//     let mut max_count = 0;

//     for word in words {
//         let mut count = 0;
//         for other_word in words {
//             if other_word == word {
//                 count += 1;
//             }
//         }
//         if count > max_count {
//             max_word = word.to_string();
//             max_count = count;
//         }
//     }

//     (max_word, max_count)
// }

// fn main() {
//     let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
//     let (word, count) = most_frequent_word(text);
//     println!("Most frequent word: \"{}\" ({} times)", word, count);
// }

// fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
//     let mut i = low;
//     while i < high {
//         *total += i;
//         i += step;
//     }
// }
// fn main() {
//     let mut result = 0;
//     sum_with_step(&mut result, 0, 100, 1);
//     println!("Sum 0 to 100, step 1: {}", result);

//     result = 0;
//     sum_with_step(&mut result, 0, 10, 2);
//     println!("Sum 0 to 10, step 2: {}", result);

//     result = 0;
//     sum_with_step(&mut result, 5, 15, 3);
//     println!("Sum 5 to 15, step 3: {}", result);
// }

// #[allow(unused_variables, unused_mut)]
// fn sum(total: &mut i32, low: i32, high: i32) {
//     // for num in low..=high {
//     //     *total += num;
//     // }

//     *total = ((high - low) * (high - low + 1)) / 2
// }

// fn main() {
//     let mut result : i32 = 0;
//     sum(&mut result, 0, 100);
//     print!("Result: {}\n", result);
// }