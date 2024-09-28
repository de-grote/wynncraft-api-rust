use crate::{
    api_request, classes::Class, deserialize_with_default, item::Profession, Identifier, Map, Set,
    World, WynnApiError, API_LOCATION,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    pub username: String,
    pub online: bool,
    pub server: Option<World>,
    pub active_character: Option<String>,
    pub uuid: String,
    pub rank: String,
    pub rank_badge: Option<String>,
    pub legacy_rank_colour: Option<LegacyRankColour>,
    pub shortened_rank: Option<String>,
    pub support_rank: SupportRank,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub veteran: bool,
    // could prob be parsed into a datetime
    pub first_join: String,
    pub last_join: String,
    // maybe change into a Duration or chrono equivalent
    /// playtime in hours
    pub playtime: f64,
    pub guild: GuildInfo,
    /// data independent of profile
    pub global_data: GlobalData,
    pub forum_link: Option<u64>,
    pub ranking: Map<String, i64>,
    pub previous_ranking: Map<String, i64>,
    pub public_profile: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LegacyRankColour {
    pub main: String,
    pub sub: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GuildInfo {
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub prefix: String,
    // into enum
    pub rank: String,
    pub rank_stars: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlobalData {
    pub wars: u64,
    pub total_level: u64,
    pub killed_mobs: u64,
    pub chests_found: u64,
    #[serde(default)]
    pub dungeons: DungeonInfo,
    #[serde(default)]
    pub raids: RaidInfo,
    pub completed_quests: u64,
    pub pvp: Pvp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DungeonInfo {
    pub total: u64,
    // maybe into enum
    pub list: Map<String, u64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RaidInfo {
    pub total: u64,
    // maybe into enum
    pub list: Map<String, u64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pvp {
    pub kills: u64,
    pub deaths: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    #[serde(rename = "type")]
    pub class_type: Class,
    pub nickname: Option<String>,
    pub level: u8,
    pub xp: u64,
    /// always between 0 and 100 even when lvl 106
    pub xp_percent: u8,
    pub total_level: u64,
    pub wars: u64,
    pub playtime: f64,
    pub mobs_killed: u64,
    pub chests_found: u64,
    pub blocks_walked: u64,
    pub items_identified: u64,
    pub logins: u64,
    pub deaths: u64,
    pub discoveries: u64,
    /// this field isnt documented for some reason
    pub pre_economy: bool,
    pub pvp: Pvp,
    // maybe into bitfield?
    pub gamemode: Set<String>,
    pub skill_points: SkillPoints,
    pub professions: Professions,
    pub dungeons: DungeonInfo,
    pub raids: RaidInfo,
    pub quests: Set<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SkillPoints {
    pub strength: u64,
    pub dexterity: u64,
    pub intelligence: u64,
    // this is the only place defence is spelled like `defense` in the api
    // in the documentation this is correct though
    #[serde(alias = "defense")]
    pub defence: u64,
    pub agility: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Professions {
    pub fishing: ProfessionInfo,
    pub woodcutting: ProfessionInfo,
    pub mining: ProfessionInfo,
    pub farming: ProfessionInfo,
    pub scribing: ProfessionInfo,
    pub jeweling: ProfessionInfo,
    pub alchemism: ProfessionInfo,
    pub cooking: ProfessionInfo,
    pub weaponsmithing: ProfessionInfo,
    pub tailoring: ProfessionInfo,
    pub woodworking: ProfessionInfo,
    pub armouring: ProfessionInfo,
}

impl Professions {
    pub fn get_profession(&self, profession: Profession) -> &ProfessionInfo {
        match profession {
            Profession::Alchemism => &self.alchemism,
            Profession::Armouring => &self.armouring,
            Profession::Cooking => &self.cooking,
            Profession::Jeweling => &self.jeweling,
            Profession::Scribing => &self.scribing,
            Profession::Tailoring => &self.tailoring,
            Profession::Weaponsmithing => &self.weaponsmithing,
            Profession::Woodworking => &self.woodworking,
            Profession::Mining => &self.mining,
            Profession::Fishing => &self.fishing,
            Profession::Farming => &self.farming,
            Profession::Woodcutting => &self.woodcutting,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProfessionInfo {
    pub level: u8,
    pub xp_percent: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullPlayerStats {
    #[serde(flatten)]
    pub player_stats: PlayerStats,
    pub characters: Map<String, Character>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharacterInfo {
    #[serde(rename = "type")]
    pub class_type: Class,
    pub nickname: Option<String>,
    pub level: u8,
    pub xp: u64,
    /// always between 0 and 100 even when lvl 106
    pub xp_percent: u8,
    pub total_level: u64,
    pub gamemode: Set<String>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Meta {
    pub died: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SupportRank {
    Vip,
    VipPlus,
    Hero,
    Champion,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Ability {
    pub coordinates: AbilityNodeCoordinate,
    #[serde(flatten)]
    pub meta: AbilityMeta,
    pub family: Set<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct AbilityNodeCoordinate {
    pub x: u8,
    pub y: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "meta")]
pub enum AbilityMeta {
    Ability {
        icon: Icon,
        page: u8,
        id: String,
    },
    Connector {
        // maybe into enum if anyone actually cares about this field
        icon: String,
        page: u8,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "format", content = "value")]
pub enum Icon {
    Attribute(AttributeIcon),
    Skin(String),
    Legacy(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttributeIcon {
    pub id: String,
    pub name: String,
    pub custom_model_data: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OnlinePlayerList {
    pub total: u64,
    pub players: Map<String, World>,
}

pub async fn player_main_stats(name_or_uuid: &str) -> Result<PlayerStats, WynnApiError> {
    api_request(&format!("{API_LOCATION}/player/{name_or_uuid}")).await
}

pub async fn player_full_stats(name_or_uuid: &str) -> Result<FullPlayerStats, WynnApiError> {
    api_request(&format!("{API_LOCATION}/player/{name_or_uuid}?fullResult")).await
}

pub async fn player_character_list(
    name_or_uuid: &str,
) -> Result<Map<String, CharacterInfo>, WynnApiError> {
    api_request(&format!("{API_LOCATION}/player/{name_or_uuid}/characters")).await
}

pub async fn player_character_data(
    name_or_uuid: &str,
    character_uuid: &str,
) -> Result<Character, WynnApiError> {
    api_request(&format!(
        "{API_LOCATION}/player/{name_or_uuid}/characters/{character_uuid}"
    ))
    .await
}

pub async fn player_character_abilities(
    name_or_uuid: &str,
    character_uuid: &str,
) -> Result<Vec<Ability>, WynnApiError> {
    api_request(&format!(
        "{API_LOCATION}/player/{name_or_uuid}/characters/{character_uuid}/abilities"
    ))
    .await
}

pub async fn online_player_list(
    identifier: Identifier,
    servers: impl IntoIterator<Item = &World>,
) -> Result<OnlinePlayerList, WynnApiError> {
    let servers_vec = servers
        .into_iter()
        .map(|x| x.0.to_string())
        .collect::<Vec<String>>();
    let servers = servers_vec.join(",");
    api_request(&format!(
        "{API_LOCATION}/player?identifier={identifier}&server={servers}"
    ))
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use std::fmt::Debug;
    use tokio;

    #[tokio::test]
    async fn player_stats() {
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

        let name = "de_grote";
        let main = player_main_stats(name).await;
        verify(&main);
        let full = player_full_stats(name).await;
        verify(&full);
        let list = player_character_list(name).await;
        verify(&list);
        let profile = "55a92635-0482-4e0a-b4ff-8284e7c8a326";
        let data = player_character_data(name, profile).await;
        verify(&data);
        let abilities = player_character_abilities(name, profile).await;
        verify(&abilities);
    }

    #[tokio::test]
    async fn online_players() {
        let players =
            online_player_list(Identifier::Username, &[World(1), World(5), World(8)]).await;
        assert!(players.is_ok());
        let uuids = online_player_list(Identifier::Uuid, &[World(1), World(5), World(8)]).await;
        assert!(uuids.is_ok());
    }
}
