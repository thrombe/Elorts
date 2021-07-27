
use serde_derive::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono;
use chrono::Datelike;// import trait to use DateTime.date().weekday() and stuff

use super::dweet::Dweet;
use super::discord::{Discord, DiscordMsg};

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

/// this is the main func here
pub fn remind(cordwebhook: String, dweekee: String) -> Result<(), Box<dyn std::error::Error>> {
    let dweet = Dweet::new(dweekee);
    let time_span: u64 = 60*15; // half the check interval time
    let mut discord = Discord::new(cordwebhook);
    // dweet.post_data(&Reminder::test_data())?;
    let mut data = match dweet.get_data::<Reminder>() {
        Ok(val) => val,
        Err(er) => panic!("{:?}", er),
    };
    // println!("{:?}", &data);
    
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


pub fn add_reminder(dweekee: String, mut date_num: Vec<u32>, time_num: Vec<u32>, message: String)
        -> Result<(), Box<dyn std::error::Error>> {
    let time_offset = (5*60 + 30)*60;
    let now = chrono::Utc::now();
    // let now_ts = now.timestamp();
    let today = (now.clone() + chrono::Duration::seconds(time_offset)).date();
    // println!("today: {:?}\nday: {:?}", &today, &today.weekday());
    
    match date_num.len() {
        1 => {
            if today.day() > date_num[0] {date_num.push(today.month()+1)} // month correction
            else {date_num.push(today.month())}
            date_num.push(today.year() as u32);
            if date_num[1] == 13 { // december to jan correction
                date_num[1] = 1;
                date_num[2] += 1;
            }
        },
        2 => date_num.push(today.year() as u32),
        3 => (),
        _ => panic!("bad input"),
    }
    
    let day = chrono::NaiveDate::from_ymd(date_num[2] as i32, date_num[1], date_num[0]);
    let time = chrono::NaiveTime::from_hms(time_num[0], time_num[1], 00);
    let datetime = chrono::NaiveDateTime::new(day, time);
    let datetime = chrono::DateTime::<chrono::Utc>::from_utc(datetime, chrono::Utc);
    let future_ts = datetime.timestamp()-time_offset;
    
    let rem = Reminder {
        time: future_ts as u64,
        message: message,
    };
    let dweet = Dweet::new(dweekee);
    let mut data = match dweet.get_data::<Reminder>() {
        Ok(val) => val,
        Err(er) => panic!("{:?}", er),
    };
    data.push(rem);
    dweet.post_data(&data)?;
    
    println!("{:?}, {:?}", day, time);
    Ok(())
}
