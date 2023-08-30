fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn longest_common_prefix(strs: Vec<String>) -> String {
    strs.into_iter()
        .reduce(|acc, curr| {
            acc.chars()
                .zip(curr.chars())
                .take_while(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect()
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = vec!["flower", "flow", "flight", "flowers"];
        let input: Vec<_> = input.iter().map(|s| s.to_string()).collect();

        assert_eq!(longest_common_prefix(input), "fl");

        let input = vec!["dog", "racecar", "car"];
        let input: Vec<_> = input.iter().map(|s| s.to_string()).collect();

        assert_eq!(longest_common_prefix(input), "");

        let input = vec!["ab", "a"];
        let input: Vec<_> = input.iter().map(|s| s.to_string()).collect();

        assert_eq!(longest_common_prefix(input), "a");
    }
}
