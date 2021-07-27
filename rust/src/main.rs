#![allow(non_snake_case)]
#![allow(dead_code)]

mod remElorts;
mod dweet;
mod discord;
mod webElorts;
mod search_and_chop;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "options")]
pub enum Opt {
    /// -c discord webhook -d dweet key
    Reminders {
        #[structopt(short = "c", long = "cordwebhook")]
        cordwebhook: String,
        
        #[structopt(short, long)]
        dweet: String,
    },
    
    /// -c discordwebhook -d dweet key -i (dweet key2 or json path) -j (if -i has json path)(bool)
    WebCheck {
        #[structopt(short, long)]
        cordwebhook: String,
        
        #[structopt(short, long)]
        dweet: String,
        
        #[structopt(short, long)]
        input: String,
        
        #[structopt(short, long)]
        json: bool,
    },
    
    /// -d dweet key --date date(mm, dd) -t time(hh, mm) -m message
    AddReminder {
        #[structopt(short, long)]
        dweet: String,
        
        #[structopt(long)]
        date: Vec<u32>,
        
        #[structopt(short, long)]
        time: Vec<u32>,
        
        #[structopt(short, long)]
        message: String,
    }
}

fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);
    
    match opt {
        Opt::Reminders{cordwebhook, dweet} => {
            remElorts::remind(cordwebhook, dweet).unwrap();
        },
        Opt::WebCheck{cordwebhook, dweet, input, json} => {
            webElorts::check(cordwebhook, dweet, input, json).unwrap();
        },
        Opt::AddReminder{dweet, date, time, message} => {
            remElorts::add_reminder(dweet, date, time, message).unwrap();
        },
    }
}
