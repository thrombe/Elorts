#![allow(non_snake_case)]
#![allow(dead_code)]

mod remind;
mod dweet;
mod discord;

fn main() {
    remind::remind().unwrap();
}
