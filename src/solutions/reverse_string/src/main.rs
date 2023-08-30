fn main() {}

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut input);
        assert_eq!(input, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
