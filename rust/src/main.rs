#![allow(non_snake_case)]
#![allow(dead_code)]

mod remind;
mod dweet;
mod discord;
mod web_check;
mod search_and_chop;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "options")]
pub struct Opt {
    #[structopt(short, long, required_unless("webcheck"))]
    remind: bool,
    
    #[structopt(short, long, required_unless("remind"))]
    webcheck: bool,

    #[structopt(short = "c", long = "cordwebhook")]
    cordwebhook: String,
    
    #[structopt(short, long)]
    dweet: String,
    
    #[structopt(long, required_unless("json"), required_unless("remind"))]
    dwee2: Option<String>,
    
    #[structopt(short, long, required_unless("dwee2"), required_unless("remind"))]
    json: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);
    
    if opt.remind {
        remind::remind(opt).unwrap();
    } else if opt.webcheck {
        web_check::elort(opt).unwrap();
    }
}

/*
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
*/