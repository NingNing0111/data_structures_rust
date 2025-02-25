#[cfg(test)]
pub mod test {

    #[test]
    fn test() {
        pub fn move_zeroes(nums: &mut Vec<i32>) {
            let mut p0 = 0;
            let mut p1 = 0; // 总是指向非0的位置
            while p0 < nums.len() {
                if nums[p0] != 0 {
                    let t = nums[p0];
                    nums[p0] = nums[p1];
                    nums[p1] = t;
                    p1 += 1;
                }
                p0 += 1;
            }
        }

        let mut input = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut input);
        assert_eq!(input, vec![1, 3, 12, 0, 0]);

        let mut input = vec![0];
        move_zeroes(&mut input);
        assert_eq!(input, vec![0]);
    }
}
