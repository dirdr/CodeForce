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

fn solve(bs: &str) -> &str {
    let bs = bs.chars().collect::<Vec<char>>();
    let len = bs.len();
    if len % 2 == 0
        &&
    bs.get(0).unwrap().to_owned() != ')'
        &&
    bs.get(len - 1).unwrap().to_owned() != '('
    {
        return "YES";
    };
    "NO"
}

fn main() {
    let t = get_number::<usize>().unwrap();
    for _ in 0..t {
        print!("{}\n", solve(get_line().unwrap().trim()));
    }
}
