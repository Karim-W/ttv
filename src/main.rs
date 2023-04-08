#![allow(dead_code)]
#![allow(unused_variables)]

use colored::Colorize;
use std::{
    collections::HashMap,
    env,
    io::{Read, Write},
    net::TcpStream,
    thread,
};
fn main() {
    dotenv::dotenv().ok();
    let server_endpoint = "irc.chat.twitch.tv";
    let server_port = 6667;
    let channel = "#xqc";
    let token = env::var("OAUTH_TOKEN").unwrap();
    let mut stream = TcpStream::connect(format!("{}:{}", server_endpoint, server_port))
        .expect("Failed to connect to sever");
    stream
        .write_all(format!("PASS {}\n", token).as_bytes())
        .expect("failed to write password");
    stream
        .write_all(format!("NICK {}\n", "censored_me").as_bytes())
        .expect("failed to write username");
    stream
        .write_all(format!("JOIN {}\n", channel).as_bytes())
        .expect("failed to join channel");
    stream
        .write_all("CAP REQ :twitch.tv/tags\n".as_bytes())
        .expect("failed to request tags");
    println!("Connected to {}", server_endpoint);
    loop {
        let mut buffer = [0; 2048];
        stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer);
        if message.contains("PRIVMSG") {
            let now = chrono::Local::now();
            let message = TwitchMessage::new(message.to_string());
            if message.is_none() {
                continue;
            }
            let message = message.unwrap();
            println!(
                "{} {} {}",
                now.format("%H:%M:%S").to_string().bright_black(),
                message.display_name.bright_blue(),
                message.message.bright_white()
            );
        }
    }
}
#[derive(Debug)]
struct TwitchMessage {
    // badge_info: String,
    // badges: String,
    // color: String,
    display_name: String,
    // emotes: String,
    // first_msg: String,
    // flags: String,
    // id: String,
    // is_mod: bool,
    // returning_chatter: bool,
    // room_id: String,
    // subscriber: bool,
    // tmi_sent_ts: String,
    // turbo: bool,
    // user_id: String,
    // user_type: String,
    message: String,
}

impl TwitchMessage {
    fn new(message: String) -> Option<Self> {
        let message = message;
        let mut tags = HashMap::new();
        let mut message = message.splitn(2, " :");
        let tag_string = message.next().unwrap();
        let message = message.next().unwrap();
        for tag in tag_string.split(';') {
            let mut tag = tag.splitn(2, '=');
            let key = tag.next().unwrap();
            let value = tag.next();
            match value {
                Some(value) => tags.insert(key, value),
                None => tags.insert(key, "n/a"),
            };
        }

        let end_of_message = message.find("\r").unwrap();
        let message = &message[..end_of_message];
        let start_of_message = message.find(":");
        if start_of_message.is_none() {
            return None;
        }
        let message = &message[start_of_message.unwrap() + 1..];
        let display_name = tags.get("display-name");
        if display_name.is_none() {
            return None;
        }
        Some(TwitchMessage {
            // badge_info: "".to_string(),
            // badges: tags.get("badges").unwrap().to_string(),
            // color: tags.get("color").unwrap().to_string(),
            display_name: display_name.unwrap().to_string(),
            // emotes: tags.get("emotes").unwrap().to_string(),
            // first_msg: tags.get("first-msg").unwrap().to_string(),
            // flags: tags.get("flags").unwrap().to_string(),
            // id: tags.get("id").unwrap().to_string(),
            // is_mod: tags.get("mod").unwrap().clone() == "1",
            // returning_chatter: tags.get("returning-chatter").unwrap().clone() == "1",
            // room_id: tags.get("room-id").unwrap().to_string(),
            // subscriber: tags.get("subscriber").unwrap().clone() == "1",
            // tmi_sent_ts: tags.get("tmi-sent-ts").unwrap().to_string(),
            // turbo: tags.get("turbo").unwrap().clone() == "1",
            // user_id: tags.get("user-id").unwrap().to_string(),
            // user_type: tags.get("user-type").unwrap().to_string(),
            message: message.to_string(),
        })
    }
}
