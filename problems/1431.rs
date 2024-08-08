use std::io;

fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = *candies.iter().max().unwrap();
    let mut resp: Vec<bool> = Vec::with_capacity(candies.len());

    for &number in &candies {
        resp.push(!(number + extra_candies < max));
    }

    resp
}

fn main() {
    let mut kids_input: String = String::new();
    io::stdin().read_line(&mut kids_input).expect("Erro de leitura do input");

    let kids: i32 = kids_input.trim().parse::<i32>().unwrap();
    let mut candies: Vec<i32> = Vec::with_capacity(kids as usize);

    for _ in 0..kids {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Erro de leitura do input");

        let valor: i32 = input.trim().parse::<i32>().unwrap();

        candies.push(valor);
    }

    let mut extra_input: String = String::new();
    io::stdin().read_line(&mut extra_input).expect("Erro de leitura do input");

    let extra_candies: i32 = extra_input.trim().parse::<i32>().unwrap();

    println!("{:?}", kids_with_candies(candies, extra_candies));
}

#[cfg(test)]
mod test {
    use crate::kids_with_candies;

    #[test]
    fn test_1() {
        assert_eq!(kids_with_candies(vec![4,2,1,1,2], 1), vec![true,false,false,false,false])
    }

    #[test]
    fn test_2() {
        assert_eq!(kids_with_candies(vec![12,1,12], 10), vec![true,false,true])
    }
}