use std::env;
use regex::Regex;
use chrono::Local;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use std::io::{stdout, Write};
use crossterm::{terminal::{Clear, ClearType}, cursor::MoveTo, ExecutableCommand};

fn run(args: &Vec<String>) {
    let (cols, rows) = crossterm::terminal::size().unwrap();

    if args.len() < 2 {
        println!("Usage: console-timer [time in seconds]");
        return;
    }

    let seconds_or_none = get_seconds(&args[1]);
    if seconds_or_none.is_none() {
        println!("Usage: console-timer [time in seconds]");
        return;
    }
    
    print_centered_message(rows, cols, "Starting!");

    let seconds_count = seconds_or_none.unwrap();
    let mut i = seconds_count;
    
    while i >= 0 {
        print_centered_message(rows, cols, &format!("{}s", i));
        sleep(Duration::from_secs(1));
        i -= 1;
    }
    
    let finish_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    print_centered_message(rows, cols, &format!("Finished after {}s at {}", seconds_count, finish_time));
}

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
        return Some(seconds * 60);
    }

    None
}

fn print_centered_message(rows: u16, cols: u16, msg: &str) {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();
    let pos_x = cols / 2 - (msg.len() as u16 / 2);
    let pos_y = rows / 2;
    stdout.execute(MoveTo(pos_x, pos_y)).unwrap();
    println!("{}", msg);
    stdout.flush().unwrap();
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: console-timer [time in seconds]");
        return;
    }
    
    run(&args);
}