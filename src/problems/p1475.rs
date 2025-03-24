#[cfg(test)]
pub mod test {

    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::<i32>::new();
        let mut ans = Vec::<i32>::new();
        let len = prices.len();
        for i in 0..len {
            while !stack.is_empty() && *(stack.get(stack.len() - 1).unwrap()) > prices[len - i - 1]
            {
                stack.pop();
            }
            if stack.is_empty() {
                ans.push(prices[len - i - 1]);
            } else {
                ans.push(prices[len - i - 1] - *(stack.get(stack.len() - 1).unwrap()));
            }
            stack.push(prices[len - i - 1]);
        }
        ans.reverse();
        return ans;
    }

    #[test]
    fn test_p1() {
        assert_eq!(final_prices(vec![8, 4, 6, 2, 3]), vec![4, 2, 4, 2, 3]);
    }
}
