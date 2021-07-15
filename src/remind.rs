#![allow(unused_imports)]

use serde_derive::{Serialize, Deserialize};

use super::dweet::Dweet;

#[derive(Serialize, Deserialize, Debug)]
pub struct Reminder {
    pub title: String,
    pub message: String,
    pub time: u64,
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