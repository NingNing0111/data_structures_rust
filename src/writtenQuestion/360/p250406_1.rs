use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse integer");

    let mut num_str = String::new();
    io::stdin()
        .read_line(&mut num_str)
        .expect("Failed to read line");
    let num_str = num_str.trim();

    println!("{}", handler(num_str, n));
}

fn handler(num_str: &str, len: usize) -> String {
    let mut min_num_str = num_str.to_string();
    for i in 0..len {
        for j in i..len {
            // flip
            let flip_str = flip(num_str, i, j);
            if flip_str < min_num_str {
                min_num_str = flip_str;
            }
        }
    }
    min_num_str
}

fn flip(num_str: &str, start: usize, end: usize) -> String {
    let mut chars: Vec<char> = num_str.chars().collect();
    let mut start = start;
    let mut end = end;
    while start < end {
        chars.swap(start, end);
        start += 1;
        end -= 1;
    }
    chars.into_iter().collect()
}
