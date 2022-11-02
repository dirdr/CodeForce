use std::{io, error::Error, str::FromStr};

pub fn get_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn get_unspace_vector() -> Option<Vec<u32>> {
    Some(get_line().ok()?.trim().chars().map(|c| c.to_digit(10)).collect::<Option<Vec<u32>>>()?)
}

fn solve(player_list: Vec<u32>) -> bool {
    let mut count = 0;
    let mut current_team = player_list.get(0).unwrap();
    for i in 0..player_list.len() {
        let curr = player_list.get(i).unwrap();
        if curr == current_team {
            count += 1;
            if count == 7 { return true; }
        } else {
            count = 1;
            current_team = curr;
        }
    }
    false
}

fn main() {
    let line = get_unspace_vector();
    match solve(line.unwrap()) {
        true => print!("YES"),
        false => print!("NO")
    }
}
