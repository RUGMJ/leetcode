fn main() {}

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        let mut n = n;

        while n % 3 == 0 {
            n /= 3;
        }

        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::is_power_of_three(27));
        assert!(!Solution::is_power_of_three(0));
        assert!(!Solution::is_power_of_three(-1));
        assert!(!Solution::is_power_of_three(43046720));
    }
}
