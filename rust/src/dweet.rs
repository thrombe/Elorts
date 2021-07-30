use reqwest;
use std::collections::HashMap;
// use serde_json::{json, to_string, from_str, from_value};
use serde_json::{from_str, from_value, to_string};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Dweet {
    // _key: String,
    get_link: String,
    post_link: String,
}

impl Dweet {
    pub fn new(dweet: String) -> Self {
        Self {
            get_link: format!("https://dweet.io/get/latest/dweet/for/{}", &dweet),
            post_link: format!("https://dweet.io/dweet/for/{}", &dweet),
            // _key: dweet,
        }
    }

    /// get the data stored in dweep and deserialise it into a vec of Reminders
    /// this func panics!! if data is not in correct format
    /// outdated doc
    pub fn get_data<T>(&self) -> Result<Vec<T>, Box<dyn std::error::Error>> 
    where T: Serialize + for<'de> Deserialize<'de> {
        
        let resp = reqwest::blocking::get(&self.get_link)?.text()?; // get string out of get request
        let resp: serde_json::Value = from_str(&resp)?; // convert string to serde json objects
        let resp = match &resp["with"][0]["content"] { // get relevent data out of it
            serde_json::Value::Object(val) => val,
            // _ => panic!("unexpected data\n{}", self.get_link),
            // how do i do better errors here?
            // _ => return Ok(vec!()),
            _ => { // idk how to properly return a error here
                "1.m".parse::<u32>()?;
                return Ok(vec!())
            }
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
    /// apprarently the slice[t] also accepts Vec<t>
    pub fn post_data<T>(&self, data: &[T]) -> Result<(), Box<dyn std::error::Error>> 
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
        let txt = res.text()?;
        if let serde_json::Value::Object(val) = from_str(&txt)? {
            if val["this"] != "succeeded" {
                panic!("{}", &txt);
            }
        } else {
            panic!("{}", &txt);
        }
        // println!("{:?}", res.headers()["content-length"]);
        // println!("{:?}", res.text());
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct MultiDweet {
    charlimit: usize,
    dweeindex: usize,
    dweekee: String,
    dweet: Dweet,
}

impl MultiDweet {
    pub fn new(dweekee: String) -> MultiDweet {
        MultiDweet {
            dweet: Dweet::new(format!("{}-0", &dweekee)),
            dweekee,
            charlimit: 9000, // idk why the site says 2k chars but accepts more than 10k
            dweeindex: 0,
        }
    }
    
    /// this is expensive cuz it clones all T and also converts them to string and counts the chars
    pub fn post_data<T>(&mut self, data: &Vec<T>) -> Result<(), Box<dyn std::error::Error>> 
    where T: Serialize + for<'de> Deserialize<'de> + Clone {
        /*
        let mut data = kata.clone();
        data.append(&mut kata.clone());
        data.append(&mut kata.clone());
        println!("count- {}", to_string(&data)?.chars().count());
        self.dweet.post_data(&data)?;
        // return Ok(());
        */
        let mut chars = 0;
        let mut start = 0;
        let mut i = 0;
        for tea in data {
            chars += to_string(tea)?.chars().count();
            if chars >= self.charlimit {
                self.dweet.post_data(&data[start..i])?;
                self.new_dweet();
                start = i;
                chars = 0;
            }
            i += 1;
        }
        self.dweet.post_data(&data[start..])?;
        
        self.dweeindex = 0;
        Ok(())
    }
    
    fn new_dweet(&mut self) {
        self.dweeindex += 1;
        self.dweet = Dweet::new(format!("{}-{}", self.dweekee, self.dweeindex));
    }
    
    /// here error is bad(no error essentially) if all get_data fails
    pub fn get_data<T>(&mut self) -> Result<Vec<T>, Box<dyn std::error::Error>> 
    where T: Serialize + for<'de> Deserialize<'de> {
        let mut data = Vec::<T>::new();
        'loup: loop {
            match self.dweet.get_data::<T>() {
                Ok(val) => data.extend(val),
                Err(_) => break 'loup,
            }
            self.new_dweet();
        }
        if self.dweeindex == 0 { // idk how to do error here properly rn
            "1.m".parse::<u32>()?;
        }
        self.dweeindex = 0;
        Ok(data)
    }
}
