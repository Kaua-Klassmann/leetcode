use std::io;
use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
    let mut resul: i32 = 0;
    let mut maior: i32 = 0;
    let map: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000)
    ]);

    for letter in s.chars().rev() {
        let valor: i32 = *map.get(&letter).unwrap();
        
        if valor >= maior {
            maior = valor;
            resul += valor;
        } else {
            resul -= valor;
        }
    }

    resul
}

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Erro de leitura do input");
    input = String::from(input.trim());

    println!("{}", {roman_to_int(input)});
}

#[cfg(test)]
mod test {
    use crate::roman_to_int;

    #[test]
    fn test_1() {
        assert_eq!(roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn test_3() {
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }
}