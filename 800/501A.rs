use std::{io, error::Error, str::FromStr};

pub fn get_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn get_vector<T>() -> Result<Vec<T>, Box<dyn Error + 'static>>
where 
    T: FromStr,
    <T as FromStr>::Err: Error + 'static
{
    Ok(get_line()?.split_whitespace().map(|c| c.parse::<T>()).collect::<Result<Vec<T>, _>>()?)
}

fn solve(m: u8, n: u8) -> u8 {
    let mut answer = 0;
    if m % 2 == 0 {
        answer = (m / 2) * n;
    } else {
        if n % 2 == 0 { 
            answer = (n / 2) * m;
        } else {
            answer = ((n - 1) / 2) * m + (m - 1) / 2;
        }
    }
    answer
}

fn main() {
    let line = get_vector::<u8>().unwrap();
    print!("{}", solve(line[0], line[1]));
}
