fn fib(n: i32) -> i32 {
    let mut dp = vec![0; (n + 1) as usize];
    fib_inner(&mut dp, n)
}

fn fib_inner(dp: &mut Vec<i32>, n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    if dp[n as usize] == 0 {
        dp[n as usize] = fib_inner(dp, n - 2) + fib_inner(dp, n - 1);
    }

    dp[n as usize]
}

fn main() {
    let out = fib(44);
    println!("{out}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
    }
}
