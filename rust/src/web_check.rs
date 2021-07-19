
use std::fs;
use serde_json::{from_str};
use serde_derive::{Serialize, Deserialize};
use reqwest;

use super::discord::{Discord, DiscordMsg};
use super::search_and_chop::search_and_chop;
use super::dweet::Dweet;
use super::Opt;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WebElort {
    name: String,
    url: String,
    search_start: String,
    search_end: String,
    text: String,
}

impl DiscordMsg for WebElort {
    fn get_msg(&self) -> String {
        format!(
"<{}>
```
[{}]
new: {}
```",
            &self.url,
            &self.name,
            &self.text,
        )
    }
}

impl WebElort {
    fn fetch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get(&self.url)?.text()?;
        let mut texts = &search_and_chop(&resp, &self.search_start, &self.search_end)[..];
        if texts.len() > 5 {texts = &texts[..5]}
        self.text = texts.join(", ");
        Ok(())
    }
}

pub fn elort(opt: Opt) -> Result<(), Box<dyn std::error::Error>> {
    let dweet = Dweet::new(opt.dweet);
    let discord = Discord::new(opt.cordwebhook);
    
    // fetch from dweet
    let mut dweelorts = match dweet.get_data::<WebElort>() {
        Ok(val) => val,
        Err(er) => panic!("{:?}", er),
    };
    // println!("{:?}", &dweelorts);
    
    // this looks very dirty cuz of both json and dweet inputs, maybe yeet the json version?
    let mut elorts: Vec<WebElort>;
    let dwee2: Dweet;
    if let Some(json) = opt.json {
        let data = fs::read_to_string(json)?;
        elorts = from_str(&data)?;
        yeet_bad_elorts(&mut elorts);
    } else {
        if let Some(val) = opt.dwee2 {
            dwee2 = Dweet::new(val);
            elorts = match dwee2.get_data::<WebElort>() {
                Ok(val) => val,
                Err(er) => panic!("{:?}", er),
            };
            dwee2.post_data(&elorts)?; // posting so it dosent despawn (24 hour despawn thing)
        } else {panic!()} // code should never reach this, but this is reqiread for compiler satisfaction
    }
    
    for i in 0..elorts.len() {
        elorts[i].fetch()?;
        // println!("{:?} - {:?}", &elorts[i].name, &elorts[i].text);
    }
    
    // anything better than O(n²) that i can do in Vec ?
    'loup: for i in 0..elorts.len() { // covering everything in elorts
        for j in 0..dweelorts.len() {
            if elorts[i].name != dweelorts[j].name {continue}
            if elorts[i].text == dweelorts[j].text {continue 'loup}
            discord.ping(&elorts[i])?;
            continue 'loup;
        }
        // not found in dweelorts
        let mut elort = elorts[i].clone();
        elort.name = format!("NEW TITLE - {}", &elort.name);
        discord.ping(&elort)?;
    }
    'loup2: for i in 0..dweelorts.len() { // covering everything in dweelorts
        for j in 0..elorts.len() {
            if elorts[j].name != dweelorts[i].name {continue}
            continue 'loup2;
        }
        // not found in elorts
        dweelorts[i].name = format!("REMOVED TITLE - {}", &dweelorts[i].name);
        discord.ping(&dweelorts[i])?;
    }
    
    dweet.post_data(&elorts)?;

    Ok(())
}

fn yeet_bad_elorts(elorts: &mut Vec<WebElort>) {
    let mut i = 0;
    while i < elorts.len() {
        if elorts[i].search_end != "" {
            i += 1;
            continue
        }
        elorts.remove(i);
    }
}