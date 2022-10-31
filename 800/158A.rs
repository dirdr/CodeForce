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

fn solve() -> u16 {
    let mut answer = 0;
    let first_line = get_vector::<u8>().unwrap(); 
    let second_line = get_vector::<u8>().unwrap(); 
    let n = first_line[0];
    let k = first_line[1];
    for i in 0..n {
        let score = second_line[i as usize];
        if score >= second_line[(k - 1) as usize] && score > 0 { answer += 1; }
    }
    answer
}

fn main() {
    println!("{}", solve())
}
