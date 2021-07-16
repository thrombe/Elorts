
use reqwest;
use serde_derive::{Serialize, Deserialize};

use super::remind::Reminder;

pub struct Discord {
    webhook: String,
}

impl Discord {
    pub fn new(webhook: String) -> Self {
        Self {webhook}
    }
    
    pub fn ping(&self, reminder: &Reminder) ->  Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        
        let message = DiscordMessage {
            content: format!(
                "``` {} ```",
                &reminder.message,
                ),
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