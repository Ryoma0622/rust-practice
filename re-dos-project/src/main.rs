use std::time::Instant;
// use std::time::Duration;
use regex::Regex;
// use std::thread;

fn check_regex(input_string: &str) {
    let start_time: Instant = Instant::now();
    // thread::sleep(Duration::from_secs(1));
    let regex: Regex = Regex::new(r"^(([a-zA-Z0-9])+)+$").unwrap();
    if regex.is_match(input_string) {
        println!("Match the regex");
    } else {
        println!("Not match the regex");
    }
    println!("Text: {}, Execution time: {:.10} seconds", input_string, start_time.elapsed().as_secs_f64());
}

fn main() {
    check_regex("abcdefghij");
    check_regex("abcdefghijklmnopqrstuvwxyz");
    check_regex("abcdefghijklmnopqrstuvwxyzABC");
    check_regex("abcdefghij@");
    check_regex("abcdefghijklmnopqrstuvwxyz@");
    check_regex("abcdefghijklmnopqrstuvwxyzA@");
    check_regex("@abcdefghijklmnopqrstuvwxyzA");
    check_regex("abcdefghijklmnopqrstuvwxyzA@B");
}
