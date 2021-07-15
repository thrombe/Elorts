use reqwest;
use std::collections::HashMap;
use serde_json::{json, to_string, from_str};

use super::remind::Reminder;

pub struct Dweet<'a> {
    key: &'a str,
    get_link: String,
    post_link: String,
}

impl<'a> Dweet<'a> {
    pub fn new(dweet: &'a str) -> Self {
        Self {
            key: dweet,
            get_link: format!("https://dweet.io/get/latest/dweet/for/{}", dweet),
            post_link: format!("https://dweet.io/dweet/for/{}", dweet),
        }
    }
    
    /// get the data stored in dweep and deserialise it into a vec of Reminders
    pub fn get_data(&self) -> Result<Vec<Reminder>, Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get(&self.get_link)?
            .text()?;
        let resp: serde_json::Value = from_str(&resp)?;
        let resp = match resp["with"][0]["content"].clone() {
            serde_json::Value::Object(val) => val,
            _ => panic!(),
        };
        // let resp2: HashMap<u64, Reminder<'_>> = from_str(&resp1).unwrap();
        
        let mut reminder_vec = Vec::new();
        for (_, rem) in resp.iter() {
            reminder_vec.append(&mut vec![ // stuffing reminders in a vec to return
                Reminder { // manually deserialising reminders from the serde_json object
                    title: match rem["title"].clone() { // i had to do clones here to save myself from madness
                        serde_json::Value::String(val) => val,
                        _ => panic!(), // how do i do better errors here?
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
        Ok(reminder_vec)
    }
    
    /// dosent do anything useful for now, but can be used to post hashmaps of Reminders in dweet
    pub fn post_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut data = HashMap::<u64, Reminder>::new(); // creating a test value for dweet
        data.insert(0, Reminder {
            title: "testle".to_string(),
            message: "sawkon these".to_string(),
            time: "bowls".to_string(),
        });
        
        let client = reqwest::blocking::Client::new();
        let res = client.post(&self.post_link)
            .json(&data)
            .send()?;
        
        Ok(())
    }
}