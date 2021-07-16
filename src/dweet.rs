use reqwest;
use std::collections::HashMap;
#[allow(unused_imports)]
use serde_json::{json, to_string, from_str, from_value};

use super::remind::Reminder;

pub struct Dweet<'a> {
    _key: &'a str,
    get_link: String,
    post_link: String,
}

impl<'a> Dweet<'a> {
    pub fn new(dweet: &'a str) -> Self {
        Self {
            _key: dweet,
            get_link: format!("https://dweet.io/get/latest/dweet/for/{}", dweet),
            post_link: format!("https://dweet.io/dweet/for/{}", dweet),
        }
    }
    
    /// get the data stored in dweep and deserialise it into a vec of Reminders
    /// this func panics!! if data is not in correct format
    pub fn get_data(&self) -> Result<Vec<Reminder>, Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get(&self.get_link)?.text()?; // get string out of get request
        let resp: serde_json::Value = from_str(&resp)?; // convert string to serde json objects
        let resp = match &resp["with"][0]["content"] { // get relevent data out of it
            serde_json::Value::Object(val) => val,
            _ => panic!(),
            // how do i do better errors here?
        };

        let mut reminder_vec: Vec<Reminder> = Vec::new();
        for rem in resp.values() { // stuffing reminders in a vec to return
            // i had to do clones here to save myself from pain
            reminder_vec.push(from_value(rem.clone())?);
        }
        Ok(reminder_vec)
    }
    
    /// used to post hashmaps of Reminders in dweet
    /// this may panic!!!
    pub fn post_data(&self, mut data: Vec<Reminder>) -> Result<(), Box<dyn std::error::Error>> {
        // .get_data expects data in a hashmap
        let mut map = HashMap::<u64, Reminder>::new();
        let mut i = 0;
        for reminder in data.drain(0..data.len()) { // drain pops the element as its used
            map.insert(i, reminder);
            i += 1;
        }
        
        let client = reqwest::blocking::Client::new();
        let res = client.post(&self.post_link)
            .json(&data)
            .send()?;
        if !res.status().is_success() {panic!()};
        
        Ok(())
    }
    
    /// upload test data in dweet (old data may be lost!!)
    #[allow(dead_code)]
    fn post_test_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut data = HashMap::<u64, Reminder>::new(); // creating a test value for dweet
        data.insert(0, Reminder {
            title: "testle".to_string(),
            message: "sawkon these".to_string(),
            time: 73737,
        });
        
        let client = reqwest::blocking::Client::new();
        let res = client.post(&self.post_link)
            .json(&data)
            .send()?;
        if !res.status().is_success() {panic!()};
        
        Ok(())
    }
}