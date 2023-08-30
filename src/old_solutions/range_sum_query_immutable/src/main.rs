#[allow(dead_code)]
struct NumArray(Vec<i32>);

#[allow(dead_code)]
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self(nums)
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.0
            .iter()
            .skip(left as usize)
            .take((right - left) as usize + 1)
            .sum()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(array.sum_range(0, 2), 1);
        assert_eq!(array.sum_range(2, 5), -1);
        assert_eq!(array.sum_range(0, 5), -3);
    }
}
