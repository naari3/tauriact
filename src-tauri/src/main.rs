#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod entities;

use crate::entities::tweet::Tweet;
use std::fs;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
              let mut test = fs::read_to_string("/mnt/c/Users/naari/Downloads/tweet.js").unwrap();
              test = test[25..].to_string();
              let tweets: Vec<Tweet> = serde_json::from_str(&test).unwrap();
              let ids: Vec<_> = tweets.iter().map(|t| t.tweet.id.0).collect();
              dbg!(ids);
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
