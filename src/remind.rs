
use serde_derive::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

use super::dweet::Dweet;
use super::discord::Discord;

#[derive(Serialize, Deserialize, Debug)]
pub struct Reminder {
    pub message: String,
    pub time: u64,
}

/// this is the main func here
pub fn remind() -> Result<(), Box<dyn std::error::Error>> {
    let dweet = Dweet::new("beso-beso-beminders");
    let time_span: u64 = 60*15; // half thebcheck interval time
    let mut discord = Discord::new(
           "https://discord.com/api/webhooks/864157339413774380/fOScRd_0ofvOrIRKr5qxYFDj5XA9GzVFzJnhWSc0UnJbIOr2ptfugevA4pPlVCcHyGFY"
           .to_string()
         );
    // dweet.post_test_data()?;
    let mut data = match dweet.get_data() {
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
    pop_reminders(&mut data, now)?;
    dweet.post_data(data)?;

    Ok(())
}

/// pops remindors which are no longer needed
fn pop_reminders(data: &mut Vec<Reminder>, now: u64)
    ->  Result<(), Box<dyn std::error::Error>> {
    let mut i = 0;
    while i < data.len() {
        if now < data[i].time {
            i += 1;
            continue
        }
        data.remove(i);
    }
    Ok(())
}




/*
//// ignore stuff after this, its just testing
fn discord_stuff() {
    
    #[derive(Serialize, Deserialize, Debug)]
    struct discord<'a> {
        content: &'a str,
    }
    
    
    let data = discord {
        content: "stuffshhsh",
    };
    
    let webhook = "https://discord.com/api/webhooks/852818463734890511/CHWDqR7OLTJtDyudDVsFnkq7vHRDgbIx2fe7PIA_h-RiYErQLpnkgzmgyuS0HQe26urp";
    let client = reqwest::blocking::Client::new();
    let res = client.post(webhook)
        .json(&data)
        .send();
    println!("{:?}", res);
    
}

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