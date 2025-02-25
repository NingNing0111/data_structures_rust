#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::new();
        for i in 0..nums.len() {
            if let Some(val) = map.get(&(target - nums[i])) {
                return vec![i as i32, *val as i32];
            }
            map.insert(nums[i], i as i32);
        }
        panic!("Not found.")
    }

    #[test]
    fn test_p1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let ans = two_sum(nums, target);
        assert_eq!(ans, vec![1, 0]);
    }
}
