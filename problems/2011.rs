use std::io;
use std::collections::HashMap;

fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut resul: i32 = 0;
    let map: HashMap<&str, i32> = HashMap::from([
        ("X++", 1),
        ("++X", 1),
        ("X--", -1),
        ("--X", -1)
    ]);

    for i in &operations{
        resul += map.get(i.as_str()).unwrap();
    }

    resul
}

fn main() {
    let mut vector: Vec<String> = Vec::new();

    while true {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Erro de leitura do input");
        input = input.trim().to_string();

        if input == "".to_string() {
            break;
        }

        vector.push(input);
    }

    println!("{}", final_value_after_operations(vector));
}

#[cfg(test)]
mod test {
    use crate::final_value_after_operations;

    #[test]
    fn test_1() {
        assert_eq!(final_value_after_operations(vec![
            "--X".to_string(),
            "X++".to_string(),
            "X++".to_string()
        ]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(final_value_after_operations(vec![
            "++X".to_string(),
            "++X".to_string(),
            "X++".to_string()
        ]), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(final_value_after_operations(vec![
            "X++".to_string(),
            "++X".to_string(),
            "--X".to_string(),
            "X--".to_string()
        ]), 0);
    }
}