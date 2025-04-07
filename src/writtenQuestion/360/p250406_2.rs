fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取失败");
    let n: usize = input.trim().parse().expect("解析数字失败");

    for _ in 0..n {
        let mut function_str = String::new();
        io::stdin().read_line(&mut function_str).expect("读取失败");
        let function_str = function_str.trim();

        if is_function(function_str) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn is_function(function_str: &str) -> bool {
    if let Some((left, right)) = function_str.split_once('=') {
        if evaluate_expression(left) == evaluate_expression(right) {
            return true;
        }
    }

    // 尝试在所有位置插入数字
    for i in 0..=function_str.len() {
        for num in 0..=9 {
            let new_str = format!("{}{}{}", &function_str[..i], num, &function_str[i..]);
            if let Some((left, right)) = new_str.split_once('=') {
                if evaluate_expression(left) == evaluate_expression(right) {
                    return true;
                }
            }
        }
    }
    false
}

fn evaluate_expression(expr: &str) -> i64 {
    expr.split('+')
        .map(|term| {
            term.split('*')
                .map(|num| num.parse::<i64>().unwrap_or(0))
                .product::<i64>()
        })
        .sum()
}
