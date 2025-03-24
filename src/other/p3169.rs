#[cfg(test)]
pub mod test {
    use std::cmp::max;

    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let mut free_day = 0;
        let mut lastet_day = 0;
        meetings.sort();
        for meeting in meetings {
            if meeting[0] > lastet_day + 1 {
                free_day += meeting[0] - lastet_day - 1;
            }
            lastet_day = max(lastet_day, meeting[1]);
        }
        free_day += days - lastet_day;
        return free_day;
    }

    #[test]
    fn test_p1() {
        let matrix = vec![vec![5, 7], vec![1, 3], vec![9, 10]];
        let ans = count_days(10, matrix);
        assert_eq!(ans, 2);
        let matrix2 = vec![vec![2, 4], vec![1, 3]];
        let ans = count_days(5, matrix2);
        assert_eq!(ans, 1);
    }
}
