#![allow(unused_imports)]

use reqwest;
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use serde_json::{json, to_string, from_str};

struct Dweet<'a> {
    key: &'a str,
    get_link: String,
    post_link: String,
}

impl<'a> Dweet<'a> {
    fn new(dweet: &'a str) -> Self {
        Self {
            key: dweet,
            get_link: format!("https://dweet.io/get/latest/dweet/for/{}", dweet),
            post_link: format!("https://dweet.io/dweet/for/{}", dweet),
        }
    }
    
    /// get the data stored in dweep and deserialise it into a vec of Reminders
    fn get_data(&self) -> Vec<Reminder> {
        let resp = reqwest::blocking::get(&self.get_link).unwrap()
            .text().unwrap();
        let resp0: serde_json::Value = from_str(&resp).unwrap();
        let resp1 = match resp0["with"][0]["content"].clone() {
            serde_json::Value::Object(val) => val,
            _ => panic!(),
        };
        // let resp2: HashMap<u64, Reminder<'_>> = from_str(&resp1).unwrap();
        
        let mut reminder_vec = Vec::new();
        for (_, rem) in resp1.iter() {
            reminder_vec.append(&mut vec![ // stuffing reminders in a vec to return
                Reminder { // manually deserialising reminders from the serde_json object
                    title: match rem["title"].clone() { // i had to do clones here to save myself from madness
                        serde_json::Value::String(val) => val,
                        _ => panic!(),
                    },
                    message: match rem["message"].clone() {
                        serde_json::Value::String(val) => val,
                        _ => panic!(),
                    },
                    time: match rem["time"].clone() {
                        serde_json::Value::String(val) => val,
                        _ => panic!(),
                    },
                }
            ]);
        }
        reminder_vec
    }
    
    /// dosent do anything useful for now, but can be used to post hashmaps of Reminders in dweet
    fn post_data(&self) {
        let mut data = HashMap::<u64, Reminder>::new(); // creating a test value for dweet
        data.insert(0, Reminder {
            title: "testle".to_string(),
            message: "sawkon these".to_string(),
            time: "bowls".to_string(),
        });
        
        let client = reqwest::blocking::Client::new();
        let res = client.post(&self.post_link)
            .json(&data)
            .send();
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Reminder {
    title: String,
    message: String,
    time: String,
}

/// this is the main func here
pub fn remind() -> Result<(), Box<dyn std::error::Error>> {
    let dweet = Dweet::new("beso-beso-beminders");
    dweet.post_data();
    let data = dweet.get_data();
    println!("{:?}", data);

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