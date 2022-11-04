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

pub fn get_number<T>() -> Result<T, Box<dyn Error + 'static>>
where 
    T: FromStr,
    <T as FromStr>::Err: Error + 'static
{
    Ok(get_line()?.trim().parse::<T>()?)
}

pub fn solve(n: u8, vec: &mut Vec<u16>) -> u8 {
    let mut left_sum: u16 = vec.iter().sum();
    let mut right_sum = 0;
    let mut count = 0;
    vec.sort();
    for i in (0..n).rev() {
        right_sum += vec[i as usize];
        left_sum -= vec[i as usize];
        count += 1; 
        if right_sum > left_sum {
            return count;
        }
    }
    count 
}

fn main() {
    let n = get_number::<u8>().unwrap();
    let mut vec = get_vector::<u16>().unwrap();
    print!("{}", solve(n, &mut vec));
}
