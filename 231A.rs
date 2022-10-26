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
 
fn solve(n: u16) -> u16 {
    let mut answer = 0;
    for _ in 0..n {
        let line_vec = get_vector::<u8>().unwrap();
        if line_vec.iter().sum::<u8>() >= 2 {
            answer += 1
        }
    }
    answer
}
 
fn main() {
    let n = get_number::<u16>().unwrap(); 
    println!("{}", solve(n))
}
