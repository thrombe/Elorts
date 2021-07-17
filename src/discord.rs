
use reqwest;
use serde_derive::{Serialize, Deserialize};

pub struct Discord {
    webhook: String,
}

impl Discord {
    pub fn new(webhook: String) -> Self {
        Self {webhook}
    }
    
    pub fn ping<T>(&self, tea: &T) ->  Result<(), Box<dyn std::error::Error>> 
    where T: DiscordMsg {
        let client = reqwest::blocking::Client::new();
        
        let message = DiscordMessage {
            content: tea.get_msg(),
        };
        let res = client.post(&self.webhook)
            .json(&message)
            .send();
        res?;
        
        Ok(())
    }
    
    pub fn rate_limit_wait(&mut self) {
        () // pass for now
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DiscordMessage {
    content: String,
}

pub trait DiscordMsg {
    fn get_msg(&self) -> String;
}