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

fn solve(m: i32, prices: &mut Vec<i32>) -> i32 {
    prices.sort();
    let mut sum = 0;
    let mut count = 0;
    prices.iter().map(|x| -x).take_while(|x| {
        if sum < sum + *x {
            sum += *x;
            count += 1;
            count <= m
        } else {
            false
        }
    }).fold(0, |acc, x| x + acc)
}

fn main() {
    let m = *get_vector::<i32>().unwrap().get(1).unwrap();
    let mut prices = get_vector::<i32>().unwrap();
    print!("{}", solve(m, &mut prices));
}
