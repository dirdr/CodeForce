use std::{io, error::Error, str::FromStr};

pub fn get_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn get_number<T>() -> Result<T, Box<dyn Error + 'static>>
where 
    T: FromStr,
    <T as FromStr>::Err: Error + 'static
{
    Ok(get_line()?.trim().parse::<T>()?)
}

fn solve(n: u8) -> Vec<String> {
    let mut answer = Vec::new();
    for _ in 0..n {
        let mut container = String::new();
        let line = String::from(get_line().unwrap().trim());
        let len = line.len();
        if len <= 10 {
            container.push_str(&line);
        } else {
            let fixed_length = len - 2;
            let mut line_it = line.chars();
            container.push(line_it.nth(0).unwrap());
            container.push_str(&fixed_length.to_string());
            container.push(line_it.last().unwrap());
        }
        answer.push(container);
    }
    answer
}

fn main() {
    let num = get_number::<u8>().unwrap();
    let answer = solve(num);
    for el in answer {
        println!("{}", el);
    }
}
