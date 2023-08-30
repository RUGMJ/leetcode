fn fizz_buzz(n: i32) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for i in 1..n + 1 {
        let mut word: String = "".to_string();

        if (i % 3) == 0 {
            word.push_str("Fizz");
        }

        if (i % 5) == 0 {
            word.push_str("Buzz");
        }

        if word.is_empty() {
            output.push(i.to_string());
        } else {
            output.push(word)
        }
    }
    output
}

fn main() {
    fizz_buzz(3);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let case = vec!["1", "2", "Fizz"];
        let case: Vec<_> = case.iter().map(|x| x.to_string()).collect();
        assert_eq!(case, fizz_buzz(case.len() as i32));

        let case = vec!["1", "2", "Fizz", "4", "Buzz"];
        let case: Vec<_> = case.iter().map(|x| x.to_string()).collect();
        assert_eq!(case, fizz_buzz(case.len() as i32));

        let case = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz",
        ];

        let case: Vec<_> = case.iter().map(|x| x.to_string()).collect();
        assert_eq!(case, fizz_buzz(case.len() as i32));
    }
}
