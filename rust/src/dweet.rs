use reqwest;
use std::collections::HashMap;
// use serde_json::{json, to_string, from_str, from_value};
use serde_json::{from_str, from_value};
use serde::{Serialize, Deserialize};

pub struct Dweet {
    _key: String,
    get_link: String,
    post_link: String,
}

impl Dweet {
    pub fn new(dweet: String) -> Self {
        Self {
            get_link: format!("https://dweet.io/get/latest/dweet/for/{}", &dweet),
            post_link: format!("https://dweet.io/dweet/for/{}", &dweet),
            _key: dweet,
        }
    }

    /// get the data stored in dweep and deserialise it into a vec of Reminders
    /// this func panics!! if data is not in correct format
    pub fn get_data<T>(&self) -> Result<Vec<T>, Box<dyn std::error::Error>> 
    where T: Serialize + for<'de> Deserialize<'de> {
        
        let resp = reqwest::blocking::get(&self.get_link)?.text()?; // get string out of get request
        let resp: serde_json::Value = from_str(&resp)?; // convert string to serde json objects
        let resp = match &resp["with"][0]["content"] { // get relevent data out of it
            serde_json::Value::Object(val) => val,
            _ => panic!("unexpected data"),
            // how do i do better errors here?
        };
        
        // tea -> an instance of T
        let mut tea_vec: Vec<T> = Vec::new();
        for tea in resp.values() { // stuffing reminders in a vec to return
            // i had to do clones here to save myself from pain
            tea_vec.push(from_value(tea.clone())?);
        }
        Ok(tea_vec)
    }
    
    /// used to post hashmaps of Reminders in dweet
    /// this may panic!!!
    pub fn post_data<T>(&self, data: &Vec<T>) -> Result<(), Box<dyn std::error::Error>> 
    where T: Serialize + for<'de> Deserialize<'de> {
        
        // .get_data expects data in a hashmap
        let mut map = HashMap::<u64, &T>::new();
        // tea -> an instance of T
        for i in 0..data.len() {
            map.insert(i as u64, &data[i]);
        }
        
        let client = reqwest::blocking::Client::new();
        let res = client.post(&self.post_link)
            .json(&map)
            .send()?;
        if !res.status().is_success() {panic!("posting data failed")};
        
        Ok(())
    }
}