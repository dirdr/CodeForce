use std::io;

fn get_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn solve(w: i32) -> bool {
    let mut left = 2;
    let mut right = w - left;
    while left <= right {
        if left % 2 == 0 && right % 2 == 0 { return true; }
        left += 2;
        right = w - left;
    } 
    false
}

fn main() {
    let binding = get_line().expect("provide a valid input");
    let input = binding.trim();
    if solve(input.parse::<i32>().expect("Need a valid number")) {
        print!("YES");
    } else { print!("NO"); }
}
