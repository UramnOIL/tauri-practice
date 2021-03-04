use serde::{Deserialize, Deserializer};
use serde::Serialize;
use std::collections::HashMap;
use std::str::FromStr;

fn format_bool<'de, D>(deserializer: D) -> Result<bool, D::Error> where D: Deserializer<'de>, {
  let s: &str = Deserializer::deserialize(deserializer)?;
  Ok(u8::from_str(s)? == 1)
}


#[derive(Serialize, Deserialize)]
pub struct User {
  id: u16,
  name: String,
  description: Option<String>,
  icon_image_id: Option<String>,
  icon_image: Option<Image>,
  created_at: String,
  updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
  id: String,
  format: String,
  #[serde(rename="type")]
  _type: u8,
  url: String,
}

#[derive(Serialize, Deserialize)]
pub struct LatestPing {
  server_id: u16,
  #[serde(deserialize_with = "format_bool")]
  is_running: bool,
  millisecond: Option<u16>,
  protocol: Option<u16>,
  version: Option<String>,
  current_player: Option<u16>,
  max_player: Option<u16>,
  created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct YesterdayStatistics {
  date: String,
  #[serde(rename="type")]
  _type: u8,
  server_id: u16,
  all_ping_count: u16,
  valid_ping_count: u16,
  average_player: f32,
  max_player: u16,
  created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Votes {
  entire: u8,
  recently: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Server {
  id: u16,
  user_id: u16,
  name: String,
  address: String,
  port: u16,
  description: Option<String>,
  color: Option<String>,
  categories: Vec<u8>,
  web_sites: HashMap<String, String>,
  top_image_id: Option<String>,
  back_image_id: Option<String>,
  #[serde(deserialize_with = "format_bool")]
  is_verified: bool,
  #[serde(deserialize_with = "format_bool")]
  is_archived: bool,
  #[serde(deserialize_with = "format_bool")]
  is_display_server: bool,
  #[serde(deserialize_with = "format_bool")]
  is_display_address: bool,
  #[serde(deserialize_with = "format_bool")]
  is_display_statistics: bool,
  created_at: String,
  updated_at: String,
  user: User,
  top_image: Option<Image>,
  back_image: Option<Image>,
  latest_ping: LatestPing,
  yesterday_statistics: YesterdayStatistics,
  votes: Votes,
}

#[derive(Deserialize, Serialize)]
pub struct StatusAndServers {
  status: u8,
  pub servers: Vec<Server>,
}

#[derive(Serialize)]
pub struct GetServersResponse {
  pub servers: Vec<Server>
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  GetServersCommand {
    callback: String,
    error: String,
  }
}
