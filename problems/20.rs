use std::io;

fn is_valid(s: String) -> bool {
    let mut vector: Vec<char> = Vec::new();

    for i in s.chars() {
        match i {
            '(' => vector.push(')'),
            '[' => vector.push(']'),
            '{' => vector.push('}'),
            ')' | ']' | '}' => if Some(i) != vector.pop() {
                return false;
            },
            _ => ()
        }
    }

    vector.is_empty()
}

fn main() {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).expect("Erro de leitura do input");

    println!("{}", is_valid(s));
}

#[cfg(test)]
mod tests {
    use crate::is_valid;

    #[test]
    fn test_1() {
        assert_eq!(is_valid(String::from("()")), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(is_valid(String::from("()[]{}")), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(is_valid(String::from("(]")), false);
    }
}