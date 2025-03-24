#[cfg(test)]
pub mod test {

    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack = Vec::<i32>::new();
        let mut sum = 0;
        for item in operations.iter() {
            println!("{:?}", stack);
            if *item == String::from("+") {
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                stack.push(num2);
                stack.push(num1);
                stack.push(num1 + num2);
            } else if *item == String::from("D") {
                let pre = stack.pop().unwrap();
                stack.push(pre);
                stack.push(pre * 2);
            } else if *item == String::from("C") {
                stack.pop();
            } else {
                stack.push(item.parse().expect("转换失败"));
            }
        }

        while !stack.is_empty() {
            let cur_num = stack.pop().unwrap();
            sum += cur_num;
        }

        return sum;
    }

    #[test]
    fn test_p1() {
        let input = vec![
            "5".to_string(),
            "-2".to_string(),
            "4".to_string(),
            "C".to_string(),
            "D".to_string(),
            "9".to_string(),
            "+".to_string(),
            "+".to_string(),
        ];
        let ans = cal_points(input);
        assert_eq!(ans, 27);
    }
}
