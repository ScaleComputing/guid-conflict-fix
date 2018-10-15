use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Copy, Clone)]
enum State {
    Normal,
    FirstHalf,
    SecondHalf,
}

fn main() -> std::io::Result<()> {
    let fname = std::env::args().nth(1).unwrap();
    let f = File::open(fname)?;
    let reader = BufReader::new(f);
    let mut state = State::Normal;

    let mut pre: Vec<String> = vec![];
    let mut post: Vec<String> = vec![];

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        state = match state {
            State::Normal => {
                if line.starts_with("<<<<") {
                    State::FirstHalf
                } else {
                    println!("{}", line);
                    state
                }
            }
            State::FirstHalf => {
                if line.starts_with("====") {
                    State::SecondHalf
                } else {
                    pre.push(line);
                    state
                }
            }
            State::SecondHalf => {
                if line.starts_with(">>>>") {
                    print_guid_as_uuid(&pre);
                    State::Normal
                } else {
                    post.push(line);
                    state
                }
            }
        }
    });

    Ok(())
}

fn print_guid_as_uuid(s: &[String]) {
    let re = Regex::new(r"(?i)guid").unwrap();

    for line in s.iter() {
        if let Some(caps) = re.captures(&line) {
            let mut line = line.clone();
            for cap in caps.iter() {
                let g_ind = cap.unwrap().start();
                let g = &line[g_ind..=g_ind];
                let u = if g == "G" { "U" } else { "u" };
                line.replace_range(g_ind..=g_ind, u);
            }
            println!("{}", line);
        } else {
            println!("{}", line);
        }
    }
}
