#[cfg(test)]
pub mod test {
    use std::{cmp, collections::HashSet};

    #[test]
    fn test() {
        pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
            let set = nums.into_iter().collect::<HashSet<_>>();
            let mut ans = 0;
            for &num in &set {
                if !set.contains(&(num - 1)) {
                    let mut cur_ans = 1;
                    let mut cur_num = num;
                    while set.contains(&(cur_num + 1)) {
                        cur_num += 1;
                        cur_ans += 1;
                    }
                    ans = cmp::max(ans, cur_ans)
                }
            }
            ans
        }

        let input = vec![100, 4, 200, 1, 3, 2];
        let ans = longest_consecutive(input);
        assert_eq!(ans, 4);
    }
}
