fn main() {}

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        sort_string(s) == sort_string(t)
    }
}

fn sort_string(s: String) -> Vec<char> {
    let mut s: Vec<_> = s.chars().collect();
    s.sort();
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }
}
