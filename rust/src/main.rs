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
    RemElorts {
        #[structopt(short = "c", long = "cordwebhook")]
        cordwebhook: String,
        
        #[structopt(short, long)]
        dweet: String,
    },
    
    /// -c discord webhook -d dweet key -j json path (optional)
    WebElorts {
        #[structopt(short, long)]
        cordwebhook: String,
        
        #[structopt(short, long)]
        dweet: String,
        
        #[structopt(short, long)]
        json: Option<String>,
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
    },
    
    /// -d dweet -j json path
    UpdateWebElorts {
        #[structopt(short, long)]
        dweet: String,
        
        #[structopt(short, long)]
        json: String,
    },
}

fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);
    
    match opt {
        Opt::RemElorts{cordwebhook, dweet} => {
            remElorts::remind(cordwebhook, dweet).unwrap();
        },
        Opt::WebElorts{cordwebhook, dweet, json} => {
            webElorts::check(cordwebhook, dweet, json).unwrap();
        },
        Opt::AddReminder{dweet, date, time, message} => {
            remElorts::add_reminder(dweet, date, time, message).unwrap();
        },
        Opt::UpdateWebElorts{dweet, json} => {
            webElorts::update(dweet, json).unwrap();
        },
    }
}
