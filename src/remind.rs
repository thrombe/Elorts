
use reqwest;
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use serde_json::{json, to_string, from_str};

pub fn remind() -> Result<(), Box<dyn std::error::Error>> {
    let dweet = "beso-beso-beminders";
    // let dweet = "berobero-botifications";
    let dweet_get = format!("https://dweet.io/get/latest/dweet/for/{}", dweet);
    let dweet_post = format!("https://dweet.io/dweet/for/{}", dweet);
    
    

    Ok(())
}


#[derive(Serialize, Deserialize, Debug)]
struct Reminder<'a> {
    title: &'a str,
    message: &'a str,
    time: &'a str,
}


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