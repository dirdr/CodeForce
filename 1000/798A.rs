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

pub fn get_vector<T>() -> Result<Vec<T>, Box<dyn Error + 'static>>
where 
    T: FromStr,
    <T as FromStr>::Err: Error + 'static
{
    Ok(get_line()?.split_whitespace().map(|c| c.parse::<T>()).collect::<Result<Vec<T>, _>>()?)
}

fn solve(line: &str) -> &str {
    let line = line.chars().collect::<Vec<char>>();
    let len = line.len();
    let mut diff = 0;
    for i in 0..len/2 {
        if line[i] != line[len - i - 1] {
            diff += 1;
        }
    }
    if diff == 0 && len % 2 != 0 {return "YES"};
    if diff == 1 {return "YES"} else {return "NO"};
}

fn main() {
    print!("{}", solve(get_line().unwrap().trim()));
}
