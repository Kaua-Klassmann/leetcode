use std::io;

fn is_palindrome(x: i32) -> bool {
    let text: String = x.to_string();

    text.chars().eq(text.chars().rev())
}

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Erro de leitura do input");

    let num: i32 = input.trim().parse::<i32>().unwrap();

    println!("{}", is_palindrome(num));
}

#[cfg(test)]
mod test {
    use crate::is_palindrome;

    #[test]
    fn test_1() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(is_palindrome(10), false);
    }
}