use std::env;
use regex::Regex;
use std::str::FromStr;
use ncurses::{clear, mvprintw, refresh};

fn get_seconds(time_string: &str) -> Option<i32> {
    let num_re = Regex::new(r"^(\d+)$").unwrap();
    if num_re.is_match(time_string) {
        let c = num_re.captures(time_string).unwrap();
        let number = i32::from_str(&c[1]).unwrap();
        return Some(number);
    }
    
    let mins_re = Regex::new(r"^(\d+)m$").unwrap();
    if mins_re.is_match(time_string) {
        let c = mins_re.captures(time_string).unwrap();
        let seconds = i32::from_str(&c[1]).unwrap();
        return Some(seconds*60);
    }

    return None;
}

fn print_centered_message(rows: &i32, cols: &i32, msg: &str) {
    clear();
    let pos_x : i32 = cols/2 - ((msg.len()/2) as i32);
    mvprintw(rows/2, posX, &msg);
    refresh();
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: console-timer [time in seconds]");
        return;
    }
}