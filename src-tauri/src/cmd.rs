use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::marker::PhantomData;

use serde::{Deserialize, Deserializer};
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::Serialize;

fn format_bool<'de, D>(deserializer: D) -> Result<bool, D::Error> where D: Deserializer<'de>, {
    let s = u8::deserialize(deserializer)?;
    Ok(s == 1)
}

struct WebSitesHashMapVisitor {
    marker: PhantomData<fn() -> HashMap<String, String>>
}

impl WebSitesHashMapVisitor {
    fn new() -> Self {
        WebSitesHashMapVisitor {
            marker: PhantomData
        }
    }
}

impl<'de> Visitor<'de> for WebSitesHashMapVisitor {
    type Value = HashMap<String, String>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("うんちぶり")
    }

    fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
    {
        Ok(HashMap::new())
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
    {
        let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(map)
    }
}

fn format_web_sites_map<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error> where D: Deserializer<'de>, {
    Ok(deserializer.deserialize_any(WebSitesHashMapVisitor::new())?)
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
    #[serde(rename = "type")]
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
    #[serde(rename = "type")]
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
    address: Option<String>,
    port: Option<u16>,
    description: Option<String>,
    color: Option<String>,
    categories: Vec<u8>,
    #[serde(deserialize_with = "format_web_sites_map")]
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
