fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn mul(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 0;
    }
    let mut result = 0;
    for _ in 0..b {
        result = add(result, a);
    }
    return result;
}

fn pow(a: i32, b: i32) -> f32 {
    if b < 0 {
        return 1 / pow(a, -b);
    }
    if b == 0 {
        return 1.0;
    }
    let mut result = 1.0;
    for _ in 0..b {
        result = mul(result, a);
    }
    return result;
}

fn main() {
    println!("3 + 2 = {}", add(3, 2));
    println!("3 * 2 = {}", mul(3, 2));
    println!("3^2 = {}", pow(3, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_add_functionality {
        use super::*;
        #[test]
        fn test_add() {
            assert_eq!(add(2, 2), 4);
        }

        #[test]
        fn test_add_multiple() {
            let test_cases = vec![
                (1, 1, 2),
                (0, 0, 0),
                (-1, 1, 0),
                (100, -50, 50)
            ];
            
            for (a, b, expected) in test_cases {
                assert_eq!(add(a, b), expected, "Failed on input ({}, {})", a, b);
            }
        }
    }

    mod test_mul_functionality {
        use super::*;
        #[test]
        fn test_mul() {
            assert_eq!(mul(0, 1), 0);
            assert_eq!(mul(1, 0), 0);
            assert_eq!(mul(3, 1), 3);
            assert_eq!(mul(2, 3), 6);
        }
    }

    mod test_pow_functionality {
        use super::*;
        #[test]
        fn test_pow() {
            assert_eq!(pow(2, 0), 1.0);
            assert_eq!(pow(2, 2), 4.0);
            assert_eq!(pow(2, 3), 8.0);
            assert_eq!(pow(5, -2), 0.04);
            assert!(f32::abs(0.333 - pow(3, -1)) < 0.0005);
        }
    }
}