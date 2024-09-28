use crate::{
    api_request, deserialize_with_default, Identifier, Map, World, WynnApiError, API_LOCATION,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    pub uuid: String,
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub prefix: String,
    pub level: u64,
    pub xp_percent: u64,
    pub territories: u64,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub wars: u64,
    pub created: String,
    pub members: GuildMembers,
    pub online: u64,
    pub banner: Option<Banner>,
    pub season_ranks: Map<String, SeasonRank>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GuildMembers {
    pub total: u64,
    pub owner: Map<String, GuildPlayerInfo>,
    pub chief: Map<String, GuildPlayerInfo>,
    pub strategist: Map<String, GuildPlayerInfo>,
    pub captain: Map<String, GuildPlayerInfo>,
    pub recruiter: Map<String, GuildPlayerInfo>,
    pub recruit: Map<String, GuildPlayerInfo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GuildPlayerInfo {
    // one of these two is always none depending on the api call
    // could maybe be expressed a bit better
    pub username: Option<String>,
    pub uuid: Option<String>,
    pub online: bool,
    pub server: Option<World>,
    pub contributed: u64,
    pub contribution_rank: u64,
    pub joined: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub base: String,
    pub tier: u64,
    pub structure: String,
    pub layers: Vec<BannerLayer>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BannerLayer {
    pub colour: String,
    pub pattern: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeasonRank {
    pub rating: u64,
    pub final_terretories: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortGuildDescription {
    pub uuid: String,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub prefix: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GuildDescription {
    pub uuid: String,
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub prefix: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Territory {
    pub guild: GuildDescription,
    pub acquired: String,
    pub location: TerritoryLocation,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TerritoryLocation {
    pub start: (i64, i64),
    pub end: (i64, i64),
}

pub async fn guild_by_name(
    guild_name: &str,
    identifier: Identifier,
) -> Result<Guild, WynnApiError> {
    api_request(&format!(
        "{API_LOCATION}/guild/{guild_name}?identifier={identifier}"
    ))
    .await
}

pub async fn guild_by_prefix(
    guild_prefix: &str,
    identifier: Identifier,
) -> Result<Guild, WynnApiError> {
    api_request(&format!(
        "{API_LOCATION}/guild/prefix/{guild_prefix}?identifier={identifier}"
    ))
    .await
}

pub async fn guild_list() -> Result<Map<String, ShortGuildDescription>, WynnApiError> {
    api_request(&format!("{API_LOCATION}/guild/list/guild")).await
}

pub async fn guild_teritories() -> Result<Map<String, Territory>, WynnApiError> {
    api_request(&format!("{API_LOCATION}/guild/list/territory")).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use std::fmt::Debug;
    use tokio;

    #[tokio::test]
    async fn guild_stats() {
        fn verify<T: Debug>(t: &Result<T, WynnApiError>) {
            match t {
                Ok(x) => {
                    dbg!(x);
                }
                Err(WynnApiError::ConnectionError(e)) => {
                    if e.status() != Some(StatusCode::from_u16(500).unwrap()) {
                        panic!("{}", e)
                    }
                }
                e => {
                    panic!("{:?}", e);
                }
            };
        }

        let name = "Pleonexia";
        let main = guild_by_name(name, Identifier::Username).await;
        let main2 = guild_by_name(name, Identifier::Uuid).await;
        verify(&main);
        verify(&main2);
        let prefix = "Plex";
        let pre = guild_by_prefix(prefix, Identifier::Username).await;
        let pre2 = guild_by_prefix(prefix, Identifier::Uuid).await;
        verify(&pre);
        verify(&pre2);
        let territories = guild_teritories().await;
        verify(&territories);
    }

    #[ignore = "really shouldn't hit this endpoint often"]
    #[tokio::test]
    async fn all_guilds() {
        let all = guild_list().await;
        assert!(all.is_ok());
    }
}
