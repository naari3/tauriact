use serde::Deserialize;

use serde::de::{self, Deserializer, Visitor};
use std::fmt;

#[derive(Debug)]
pub struct StrToInt(pub i64);

impl<'de> Deserialize<'de> for StrToInt {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct IdVisitor;

    impl<'de> Visitor<'de> for IdVisitor {
      type Value = StrToInt;

      fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("user ID as a number or string")
      }

      fn visit_i64<E>(self, id: i64) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(StrToInt(id))
      }

      fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        id.parse().map(StrToInt).map_err(de::Error::custom)
      }
    }

    deserializer.deserialize_any(IdVisitor)
  }
}

#[derive(Deserialize, Debug)]
pub struct Tweet {
  pub tweet: TweetBody,
}

#[derive(Deserialize, Debug)]
pub struct TweetBody {
  pub retweeted: bool,
  pub source: String,
  pub entities: Entities,
  pub display_text_range: Vec<String>,
  pub favorite_count: String,
  pub id: StrToInt,
  pub id_str: String,
  pub truncated: bool,
  pub created_at: String,
  pub favorited: bool,
  pub full_text: String,
  pub lang: String,
  pub in_reply_to_status_id: Option<StrToInt>,
  pub in_reply_to_status_id_str: Option<String>,
  pub in_reply_to_user_id: Option<StrToInt>,
  pub in_reply_to_user_id_str: Option<String>,
  pub in_reply_to_screen_name: Option<String>,
  pub possibly_sensitive: Option<bool>,
  pub extended_entities: Option<Entities>,
  pub withheld_in_countries: Option<Vec<String>>,
  pub withheld_copyright: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Entities {
  pub hashtags: Option<Vec<Hashtag>>,
  pub symbols: Option<Vec<Symbol>>,
  pub user_mentions: Option<Vec<UserMention>>,
  pub media: Option<Vec<Media>>,
  pub urls: Option<Vec<Url>>,
}

#[derive(Deserialize, Debug)]
pub struct Hashtag {
  pub text: String,
  pub indices: Vec<StrToInt>,
}

#[derive(Deserialize, Debug)]
pub struct Symbol {
  pub text: String,
  pub indices: Vec<StrToInt>,
}

#[derive(Deserialize, Debug)]
pub struct UserMention {
  pub screen_name: String,
  pub name: String,
  pub id: StrToInt,
  pub id_str: String,
  pub indices: Vec<StrToInt>,
}

#[derive(Deserialize, Debug)]
pub struct Media {
  pub display_url: String,
  pub expanded_url: String,
  pub id: StrToInt,
  pub id_str: String,
  pub indices: Vec<StrToInt>,
  pub media_url: String,
  pub media_url_https: String,
  pub sizes: Sizes,
  pub source_status_id: Option<StrToInt>,
  pub source_status_id_str: Option<String>,
  #[serde(rename = "type")]
  pub type_name: String,
  pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Url {
  pub url: String,
  pub expanded_url: String,
  pub display_url: String,
  pub indices: Vec<StrToInt>,
}

#[derive(Deserialize, Debug)]
pub struct Poll {
  pub options: Vec<PollOption>,
  pub end_datetime: String,
  pub duration_minutes: String,
}

#[derive(Deserialize, Debug)]
pub struct Sizes {
  pub thumb: Option<Size>,
  pub large: Option<Size>,
  pub small: Option<Size>,
  pub medium: Option<Size>,
}

#[derive(Deserialize, Debug)]
pub struct Size {
  pub w: StrToInt,
  pub h: StrToInt,
  pub resize: String,
}

#[derive(Deserialize, Debug)]
pub struct PollOption {
  pub position: i32,
  pub text: String,
}
