use std::time::Instant;
use regex::Regex;

fn check_regex(input_string: &str) {
    let start_time = Instant::now();
    let regex = Regex::new(r"^(([a-zA-Z0-9])+)+$").unwrap();
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
