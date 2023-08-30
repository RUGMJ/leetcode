fn main() {}

#[allow(dead_code)]
fn reverse_string(s: &mut Vec<char>) {
    s.reverse();
}

#[cfg(test)]
mod tests {
    use crate::reverse_string;

    #[test]
    fn test() {
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut input);
        assert_eq!(input, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
