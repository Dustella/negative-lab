use std::io::stdin;
use std::io::BufRead;

fn get_char(state: &mut u16, cha: char) {
    match state {
        0 => *state = 1,
        12 => match cha {
            '0'..='9' | 'A'..='Z' => *state += 1,
            _ => *state = 0,
        },
        13 => match cha {
            '0'..='9' | 'A'..='Z' => *state = 13,
            '.' => *state += 1,
            'e' => *state = 16,
            _ => *state = 0,
        },
        14 => match cha {
            '0'..='9' => *state += 1,
            _ => *state = 0,
        },
        15 => match cha {
            'e' => *state += 1,
            '0'..='9' => *state = 15,
            _ => *state = 0,
        },
        16 => match cha {
            '+' | '-' => *state += 1,
            '0'..='9' => *state = 18,
            _ => *state = 0,
        },
        17 => match cha {
            '0'..='9' => *state += 1,
            _ => *state = 0,
        },
        18 => match cha {
            '0'..='9' => *state = 18,
            _ => *state = 0,
        },
        _ => {
            // err
        }
    }
}

#[warn(dead_code)]
pub fn main() {
    //  input from terminal, into 'word'
    let stdin = stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut err_location = 0;
        let mut state = 12;
        for (index, cha) in line.chars().enumerate() {
            get_char(&mut state, cha);
            if state == 0 {
                err_location = index
            }
        }
        if state == 0 || state == 1 {
            println!("Err at {}", err_location)
        } else {
            println!("Ok")
        }
    }
}
