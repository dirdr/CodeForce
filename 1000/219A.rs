use std::{io, error::Error, str::FromStr, collections::HashMap};

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

pub fn get_string() -> Result<String, Box<dyn Error + 'static>> {
    Ok(get_line()?.trim().to_owned())
}

pub fn solve(k: usize, str: String) -> String {
    let mut map = HashMap::new();
    str.chars().for_each(|c| {
        map.entry(c.to_string()).and_modify(|e| *e += 1).or_insert(1);
    });
    if map.iter().any(|(_,val)| val.to_owned() % k != 0) {return String::from("-1")};
    let len = map.len();
    map.iter()
        .map(|(key,val)| key.to_owned().repeat(*val / k))
        .cycle()
        .take(len * k)
        .collect::<String>()
}

pub fn main() {
    let k = get_number::<usize>().unwrap();
    let str = get_string().unwrap();
    print!("{}", solve(k, str.clone()));
}
