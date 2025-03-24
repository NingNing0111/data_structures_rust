#[cfg(test)]
pub mod test {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..n + 1 {
            dp[i as usize] = dp[(i - 1) as usize] + dp[(i - 2) as usize];
        }
        return dp[n as usize];
    }

    #[test]
    fn test_p1() {
        let res = climb_stairs(44);
        println!("{}", res)
    }
}
