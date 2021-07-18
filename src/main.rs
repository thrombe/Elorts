#![allow(non_snake_case)]
#![allow(dead_code)]

mod remind;
mod dweet;
mod discord;
mod web_check;
mod search_and_chop;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match &args[1][..] {
            "remind" => remind::remind().unwrap(),
            "web_check" => web_check::elort().unwrap(),
            _ => panic!("args - remind or web_check"),
        }
    } else {
        panic!("give args plz")
    }
}
