fn main() {
    println!("Hello, world!");
    for i in 0..47 {
        climb_stairs(i);
    }
}

/// Each time you can climb `1` or `2` steps
fn climb_stairs(n: i32) -> i32 {
    let mut out = (1, 0);
    for _ in 0..n {
        out = (out.1 + out.0, out.0)
    }

    out.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_climb_stairs() {
        let test_cases = [(1, 1), (2, 2), (3, 3), (4, 5), (5, 8), (6, 13), (7, 21)];

        for &(n, expected) in &test_cases {
            let result = climb_stairs(n);
            assert_eq!(result, expected);
        }
    }
}
