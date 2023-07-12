fn is_palindrome(num: u64) -> bool {
    let num_str = num.to_string();
    let num_str_rev: String = num_str.chars().rev().collect();
    num_str == num_str_rev
}

// Find the largest palindrome that is the product of two three-digit numbers.
fn find_largest_palindrome_three_digits() -> u64 {
    let mut palindromes = vec![];
    for x in (100..=999).rev() {
        for y in (100..=999).rev() {
            if is_palindrome(x * y) {
                palindromes.push(x * y);
            }
        }
    }

    *palindromes.iter().max().unwrap()
}

fn main() {
    println!(
        "The largest palindrome that is the product of two three-digit numbers is: {}",
        find_largest_palindrome_three_digits()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(12321), true);
        assert_eq!(is_palindrome(12345), false);
        assert_eq!(is_palindrome(123321), true);
        assert_eq!(is_palindrome(123456), false);
    }

    #[test]
    fn test_find_largest_palindrome_three_digits() {
        assert_eq!(find_largest_palindrome_three_digits(), 906609);
    }
}
