fn main() {
    is_anagram("".to_string(), "".to_string());
}

fn is_anagram(s: String, t: String) -> bool {
    sort_string(s) == sort_string(t)
}

fn sort_string(s: String) -> Vec<char> {
    let mut s: Vec<_> = s.chars().collect();
    s.sort();
    s
}

#[cfg(test)]
mod tests {
    use crate::is_anagram;

    #[test]
    fn test() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }
}
