fn main() {}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (index_a, num_a) in nums.iter().enumerate() {
            for (index_b, num_b) in nums.iter().enumerate() {
                if index_a == index_b {
                    continue;
                }

                if num_a + num_b == target {
                    return vec![index_a as i32, index_b as i32];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);

        let nums = vec![3, 2, 4];
        let target = 6;

        assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);

        let nums = vec![3, 3];
        let target = 6;

        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}
