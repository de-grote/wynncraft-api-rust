use crate::{
    api_request, deserialize_with_default, guild::TerritoryLocation, item::Item, Map, WynnApiError,
    API_LOCATION,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub query: String,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub players: Map<String, String>,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub guilds: Map<String, GuildSearchInfo>,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub guilds_prefix: Map<String, GuildSearchInfo>,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub territories: Map<String, TerritoryLocation>,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub discoveries: Map<String, DiscoveryLocation>,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub items: Map<String, Item>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GuildSearchInfo {
    pub name: String,
    pub prefix: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DiscoveryLocation {
    pub start: (i64, i64, i64),
    pub end: (i64, i64, i64),
}

pub async fn search(query: &str) -> Result<SearchResult, WynnApiError> {
    api_request(&format!("{API_LOCATION}/search/{query}")).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn searching() {
        let bob = search("bob").await;
        assert!(bob.is_ok());
    }
}
