
use serde_derive::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono;
use chrono::Datelike;// import trait to use DateTime.date().weekday() and stuff
use structopt::StructOpt;

use super::dweet::MultiDweet;
use super::discord::{Discord, DiscordMsg};
use super::printdebug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Reminder {
    pub message: String,
    pub time: u64,
}

impl DiscordMsg for &Reminder {
    fn get_msg(&self) -> String {
        format!(
            "``` {} ```",
            &self.message,
            )
    }
}

impl Reminder {
    pub fn test_data() -> Vec<Reminder> {
        let mut data = Vec::<Reminder>::new(); // creating a test value for dweet
        data.push(Reminder {
            message: "sawkon these".to_string(),
            time: !(1 as u32) as u64, // u64 cant be stored on json? idk but this is interpreted as float in serde
        });
        data.push(Reminder {
            message: "sawkon these".to_string(),
            time: 73737,
        });
        data
    }
}

/// -c discord webhook -d dweet key
#[derive(Debug, StructOpt)]
pub struct RemElorts {
    #[structopt(short, long)]
    cordwebhook: String,
    
    #[structopt(short, long)]
    dweet: String,
}

impl RemElorts {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut dweet = MultiDweet::new(self.dweet.clone());
        let time_span: u64 = 60*15; // half the check interval time
        let mut discord = Discord::new(self.cordwebhook.clone());
        let mut data = match dweet.get_data::<Reminder>() {
            Ok(val) => val,
            Err(er) => panic!("{:?}", er),
        };
        printdebug!(&data);

        let mut now: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        now += time_span;
        for reminder in &data {
            if now < reminder.time {continue}
            discord.rate_limit_wait(); // not implimented yet
            discord.ping(&reminder)?;
        }
        pop_old_reminders(&mut data, now);
        dweet.post_data(&data)?;

        Ok(())
    }
}

fn pop_old_reminders(data: &mut Vec<Reminder>, now: u64) {
    let mut i = 0;
    while i < data.len() {
        if now < data[i].time {
            i += 1;
            continue
        }
        data.remove(i);
    }
}

/// -d dweet key --date date(mm, dd) -t time(hh, mm) -m message
#[derive(Debug, StructOpt)]
pub struct AddReminder {
    #[structopt(short, long)]
    dweet: String,
    
    #[structopt(long)]
    date: Vec<u32>,
    
    #[structopt(short, long)]
    time: Vec<u32>,
    
    #[structopt(short, long)]
    message: String,
}

impl AddReminder {
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let time_offset = (5*60 + 30)*60;
        let now = chrono::Utc::now();
        let today = (now.clone() + chrono::Duration::seconds(time_offset)).date();

        match self.date.len() {
            1 => {
                if today.day() > self.date[0] {self.date.push(today.month()+1)} // month correction
                else {self.date.push(today.month())}
                self.date.push(today.year() as u32);
                if self.date[1] == 13 { // december to jan correction
                    self.date[1] = 1;
                    self.date[2] += 1;
                }
            },
            2 => self.date.push(today.year() as u32),
            3 => (),
            _ => panic!("bad input"),
        }

        let day = chrono::NaiveDate::from_ymd(self.date[2] as i32, self.date[1], self.date[0]);
        let time = chrono::NaiveTime::from_hms(self.time[0], self.time[1], 00);
        let datetime = chrono::NaiveDateTime::new(day, time);
        let datetime = chrono::DateTime::<chrono::Utc>::from_utc(datetime, chrono::Utc);
        let future_ts = datetime.timestamp()-time_offset;

        let rem = Reminder {
            time: future_ts as u64,
            message: self.message.clone(),
        };
        let mut dweet = MultiDweet::new(self.dweet.clone());
        let mut data = match dweet.get_data::<Reminder>() {
            Ok(val) => val,
            Err(er) => panic!("{:?}", er),
        };
        data.push(rem);
        dweet.post_data(&data)?;

        println!("{:?}, {:?}", day, time);
        Ok(())
    }
}
