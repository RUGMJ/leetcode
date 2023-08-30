fn main() {
    println!("Hello, world!");
}

fn is_palindrome(s: String) -> bool {
    let clean = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());

    clean.clone().eq(clean.rev())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
        assert!(!is_palindrome("race a car".to_string()));
        assert!(is_palindrome(" ".to_string()));
    }
}
