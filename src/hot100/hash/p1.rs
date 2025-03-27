#[cfg(test)]
pub mod test_1 {
    use std::collections::HashMap;

    struct Solution;

    impl Solution {
        // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //     for i in 0..nums.len() {
        //         for j in i + 1..nums.len() {
        //             if nums[i] + nums[j] == target {
        //                 return vec![i as i32, j as i32];
        //             }
        //         }
        //     }
        //     return vec![-1, -1];
        // }
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut map = HashMap::<i32, i32>::new();
            for i in 0..nums.len() {
                if let Some(value) = map.get(&(target - nums[i])) {
                    return vec![i as i32, *value as i32];
                }
                map.insert(nums[i], i as i32);
            }
            return vec![-1, -1];
        }
    }

    #[test]
    fn test_p1() {
        assert_eq!(vec![1, 0], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }
}
