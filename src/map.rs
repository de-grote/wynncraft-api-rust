use crate::{api_request, deserialize_from_string, World, WynnApiError, API_LOCATION};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Quests {
    pub quests: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkerLocation {
    pub name: String,
    pub icon: String,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub x: i64,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub y: i64,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub z: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLocation {
    pub uuid: String,
    pub name: String,
    pub nickname: Option<String>,
    pub server: Option<World>,
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub friends: Vec<FriendLocation>,
    pub party: Vec<FriendLocation>,
    pub guild: Vec<FriendLocation>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FriendLocation {
    pub uuid: String,
    pub name: String,
    pub nickname: Option<String>,
    pub server: Option<World>,
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub async fn marker_locations() -> Result<Vec<MarkerLocation>, WynnApiError> {
    api_request(&format!("{API_LOCATION}/map/locations/markers")).await
}

pub async fn player_location() -> Result<Vec<PlayerLocation>, WynnApiError> {
    api_request(&format!("{API_LOCATION}/map/locations/player")).await
}

pub async fn quest_count() -> Result<Quests, WynnApiError> {
    api_request(&format!("{API_LOCATION}/map/quests")).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn map_endpoints() {
        let markers = marker_locations().await;
        assert!(markers.is_ok());
        let player = player_location().await;
        assert!(player.is_ok());
        let quests = quest_count().await;
        assert!(quests.is_ok());
    }
}
