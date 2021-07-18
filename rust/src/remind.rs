
use serde_derive::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

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
pub fn remind() -> Result<(), Box<dyn std::error::Error>> {
    let dweet = Dweet::new("beso-beso-beminders");
    let time_span: u64 = 60*15; // half thebcheck interval time
    let mut discord = Discord::new(
           "https://discord.com/api/webhooks/864157339413774380/fOScRd_0ofvOrIRKr5qxYFDj5XA9GzVFzJnhWSc0UnJbIOr2ptfugevA4pPlVCcHyGFY"
           .to_string()
         );
    // dweet.post_data(Reminder::test_data())?;
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
    dweet.post_data(data)?;

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




/*
//// ignore stuff after this, its just testing
fn eg_ser_deser() -> Result<(), Box<dyn std::error::Error>> {
    let vec = vec![ // create whatever you wanna serialise to string
        Reminder {
            title: "testRem",
            message: "reminder to koff",
            time: "68419",
        }
    ];
    let eg_json = to_string(&vec)?; // deserialize objects to string
    println!("{:?}", eg_json);
    
    let eg_deser: Vec<Reminder> = from_str(&eg_json).unwrap(); // serialise objects from string
    println!("{:?}", eg_deser);
    
    Ok(())
}
*/