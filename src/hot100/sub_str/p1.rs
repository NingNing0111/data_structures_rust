#[cfg(test)]
pub mod test {

    #[test]
    fn test() {
        pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
            let mut ans = 0;
            let mut sum = Vec::<i32>::new();
            // sum[i] = sum[i-1] + nums[i]
            sum.push(0);
            // 前i个元素之和
            for i in 1..(nums.len() + 1) {
                sum.push(sum[i - 1] + nums[i - 1]);
            }
            for i in 0..nums.len() {
                for j in i..nums.len() {
                    // println!("{}->{} sum: {}", i, j, sum[j + 1] - sum[i]);
                    if sum[j + 1] - sum[i] == k {
                        ans += 1;
                    }
                }
            }
            ans
        }

        assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
