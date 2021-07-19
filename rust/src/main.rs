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
pub enum Opt {
    Reminders {
        #[structopt(short = "c", long = "cordwebhook")]
        cordwebhook: String,
        
        #[structopt(short, long)]
        dweet: String,
    },
    WebCheck {
        #[structopt(short, long)]
        cordwebhook: String,
        
        #[structopt(short, long)]
        dweet: String,
        
        #[structopt(short, long)]
        input: String,
        
        #[structopt(short, long)]
        json: bool,
    }
}

fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);
    
    match opt {
        Opt::Reminders{cordwebhook, dweet} => {
            remind::remind(cordwebhook, dweet).unwrap();
        }
        Opt::WebCheck{cordwebhook, dweet, input, json} => {
            web_check::elort(cordwebhook, dweet, input, json).unwrap();
        }
    }
}
