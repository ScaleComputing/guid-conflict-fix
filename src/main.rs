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
    let f = File::open("test.cpp")?;
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
            for cap in caps.iter() {
                let guid_start = cap.unwrap().start();
                let (fh, sh) = line.split_at(guid_start);
                let (g, sh) = sh.split_at(1);
                let u = if g == "G" { "U" } else { "u" };

                println!("{}{}{}", fh, u, sh);
                // println!("{} {:?}", line, cap.map(|cap| cap.start()));
            }
        } else {
            println!("{}", line);
        }
    }
}
