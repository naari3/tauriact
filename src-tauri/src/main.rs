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
            }
            GetTweetCount { path, callback } => {
              let mut test = fs::read_to_string(path).unwrap();
              test = test[25..].to_string();
              let mut tweets: Vec<Tweet> = serde_json::from_str(&test).unwrap();
              tweets.sort_by_key(|a| a.tweet.created_at.0);
              _webview.eval(&format!("window[\"{}\"]({:?})", callback, tweets.len())).unwrap();
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
