
use std::fs;
use serde_json::{from_str};
use serde_derive::{Serialize, Deserialize};
use reqwest;

use super::discord::{Discord, DiscordMsg};
use super::search_and_chop::search_and_chop;
use super::dweet::MultiDweet;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WebElort {
    name: String,
    url: String,
    search_starts: Vec<String>,
    search_ends: Vec<String>,
    texts: Vec<Vec<String>>,
    texts_reverse: bool, // choose n items from the start(default) or end of the webpage
    texts_n: usize,
    message: String,
}

impl DiscordMsg for WebElort {
    fn get_msg(&self) -> String {
        format!(
"<{}>
```
[{}]
{}
```",
            &self.url,
            &self.name,
            &self.message,
        )
    }
}

impl WebElort {
    fn fetch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get(&self.url)?.text()?;
        self.texts = vec![vec!(); self.search_ends.len()];
        self.message = "".to_string();
        for i in 0..self.search_ends.len() {
            for txt in search_and_chop(&resp, &self.search_starts[i], &self.search_ends[i]) {
                self.texts[i].push(txt.to_owned());
            }
            if self.texts_reverse {self.texts[i].reverse()}
            let message: String;
            if self.texts[i].len() > self.texts_n {
                message = self.texts[i][..self.texts_n].join(", ");
            } else {
                message = self.texts[i].join(", ");
            }
            self.message += &("new: ".to_owned() + &message).to_owned();
        }
        Ok(())
    }
    
    fn check_update(&mut self, other: &Self) -> bool {
        if self.name != other.name {panic!()}
        if self.message == other.message {return false}
        // can do more checks here in future
        true
    }
}

pub fn check(cordwebhook: String, dweekee: String, json: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut dweet = MultiDweet::new(dweekee.clone());
    // println!("{:?}\n", &dweet);
    let discord = Discord::new(cordwebhook);
    
    // fetch from dweet
    let mut dweelorts = match dweet.get_data::<WebElort>() {
        Ok(val) => val,
        Err(_) => Vec::new(),
    };
    // println!("{:?}\n", &dweelorts);
    
    let mut elorts: Vec<WebElort>;
    match json {
        Some(input) => {
            let data = fs::read_to_string(input)?;
            elorts = from_str(&data)?;
            yeet_bad_elorts(&mut elorts);
        },
        None => {
            let mut dwee2 = MultiDweet::new(format!("{}-json", &dweekee));
            elorts = match dwee2.get_data::<WebElort>() {
                Ok(val) => val,
                Err(er) => panic!("{:?}", er),
            };
            dwee2.post_data(&elorts)?; // posting so it dosent despawn (24 hour despawn thing)
        },
    }
    // println!("{:?}\n", &elorts);
    
    for i in 0..elorts.len() {
        elorts[i].fetch()?;
        // println!("{:?} - {:?}", &elorts[i].name, &elorts[i].text);
    }
    
    // anything better than O(n²) that i can do in Vec ?
    'loup: for i in 0..elorts.len() { // covering everything in elorts
        for j in 0..dweelorts.len() {
            if elorts[i].name != dweelorts[j].name {continue}
            if elorts[i].check_update(&dweelorts[j]) {
                discord.ping(&elorts[i])?;
            }
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
    
    // println!("{:?}\n", &elorts);
    dweet.post_data(&elorts)?;

    Ok(())
}

fn yeet_bad_elorts(elorts: &mut Vec<WebElort>) {
    let mut i = 0;
    while i < elorts.len() {
        if elorts[i].search_ends[0] != "" {
            i += 1;
            continue
        }
        elorts.remove(i);
    }
}

pub fn update(dweekee: String, json: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut dweet = MultiDweet::new(format!("{}-json", dweekee));
    let data = fs::read_to_string(json)?;
    let mut elorts = from_str(&data)?;
    yeet_bad_elorts(&mut elorts);
    dweet.post_data(&elorts)?;
    Ok(())
}