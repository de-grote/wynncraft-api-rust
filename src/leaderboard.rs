use crate::{
    api_request,
    classes::Class,
    deserialize_with_default,
    guild::Banner,
    player::{LegacyRankColour, SupportRank},
    WynnApiError, API_LOCATION,
};
use serde::{
    de::{DeserializeOwned, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::{
    collections::BTreeMap,
    fmt::{self, Display},
    marker::PhantomData,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LbGuild {
    pub uuid: String,
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub prefix: String,
    pub level: u64,
    pub xp: u64,
    pub members: u64,
    pub territories: u64,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub wars: u64,
    pub created: String,
    pub banner: Option<Banner>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LbPlayerGlobal {
    pub name: String,
    pub uuid: String,
    pub score: i64,
    pub previous_ranking: i64,
    pub metadata: PlayerMetaData,
    pub rank: String,
    pub rank_badge: Option<String>,
    pub support_rank: SupportRank,
    pub legacy_rank_colour: Option<LegacyRankColour>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMetaData {
    /// Some endpoints just don't return this value even when documented that they should
    pub xp: Option<u64>,
    pub playtime: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RaidMetaData {
    pub completions: u64,
    pub gambits: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LbPlayerProfile {
    pub name: String,
    pub uuid: String,
    pub score: i64,
    pub previous_ranking: i64,
    pub metadata: PlayerMetaData,
    pub character_uuid: String,
    pub character_type: Class,
    pub rank: String,
    pub rank_badge: Option<String>,
    pub support_rank: SupportRank,
    pub legacy_rank_colour: Option<LegacyRankColour>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LbRaidPlayer {
    pub name: String,
    pub uuid: String,
    pub score: i64,
    pub previous_ranking: i64,
    pub metadata: RaidMetaData,
    pub rank: String,
    pub rank_badge: Option<String>,
    pub support_rank: SupportRank,
    pub legacy_rank_colour: Option<LegacyRankColour>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LbRaidGuild {
    pub name: String,
    pub uuid: String,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub score: u64,
    pub previous_ranking: i64,
    pub metadata: RaidMetaData,
    pub banner: Option<Banner>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum GuildLbType {
    GuildLevel,
    GuildTerritories,
    GuildWars,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PlayerProfileLbType {
    WoodcuttingLevel,
    MiningLevel,
    FishingLevel,
    FarmingLevel,
    AlchemismLevel,
    ArmouringLevel,
    CookingLevel,
    JewelingLevel,
    ScribingLevel,
    TailoringLevel,
    WeaponsmithingLevel,
    WoodworkingLevel,
    PlayerContent,
    CombatSoloLevel,
    ProfessionsSoloLevel,
    TotalSoloLevel,
    HardcoreLegacyLevel,
    IronmanContent,
    UltimateIronmanContent,
    HardcoreContent,
    CraftsmanContent,
    HuntedContent,
    HuicContent,
    HuichContent,
    HichContent,
    HicContent,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PlayerGlobalLbType {
    ProfessionsGlobalLevel,
    CombatGlobalLevel,
    TotalGlobalLevel,
    GlobalPlayerContent,
    NogCompletion,
    TccCompletion,
    NolCompletion,
    WarsCompletion,
    TnaCompletion,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum RaidPlayerLbType {
    NogSrPlayers,
    NolSrPlayers,
    TccSrPlayers,
    TnaSrPlayers,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum RaidGuildLbType {
    NogSrGuilds,
    NolSrGuilds,
    TccSrGuilds,
    TnaSrGuilds,
}

macro_rules! display {
    ($t:ty) => {
        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let string = serde_json::to_string(&self)
                    .map_err(|_| std::fmt::Error)?
                    .trim_matches('"')
                    .to_owned();
                f.write_str(&string)
            }
        }
    };
}

display!(GuildLbType);
display!(PlayerProfileLbType);
display!(PlayerGlobalLbType);
display!(RaidPlayerLbType);
display!(RaidGuildLbType);

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct Leaderboard<T: DeserializeOwned> {
    #[serde(deserialize_with = "deserialize_to_vec")]
    pub leaderboard: Vec<T>,
}

impl<T: DeserializeOwned> Leaderboard<T> {
    /// gets the value at the ranking of `n`
    pub fn get_ranking(&self, n: usize) -> Option<&T> {
        self.leaderboard.get(n.wrapping_sub(1))
    }
}

pub(crate) fn deserialize_to_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct MapToVecVisitor<T> {
        _ph: PhantomData<T>,
    }

    impl<'de, T> Visitor<'de> for MapToVecVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with string keys representing indices")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            // because BTreeMaps store items based on ordering, we can just put all items in a BTree and pull them out in order
            let mut value_map: BTreeMap<usize, T> = BTreeMap::new();

            while let Some((key, value)) = map.next_entry::<String, T>()? {
                let index = key.parse::<usize>().map_err(serde::de::Error::custom)?;

                if let Some(_duplicate) = value_map.insert(index, value) {
                    return Err(serde::de::Error::duplicate_field(
                        "a number was covered twice",
                    ));
                }
            }

            if let Some((first, _)) = value_map.first_key_value() {
                if let Some((last, _)) = value_map.last_key_value() {
                    let diff = last - first;
                    if diff + 1 != value_map.len() {
                        return Err(serde::de::Error::custom(
                            "not all fields in the range were covered",
                        ));
                    }
                }
            }

            Ok(value_map.into_values().collect())
        }
    }

    deserializer.deserialize_map(MapToVecVisitor { _ph: PhantomData })
}

pub async fn leaderboard_guild(
    player_profile: GuildLbType,
    limit: u16,
) -> Result<Leaderboard<LbGuild>, WynnApiError> {
    api_request(&format!(
        "{API_LOCATION}/leaderboards/{player_profile}?resultLimit={limit}"
    ))
    .await
}

pub async fn leaderboard_player_profile(
    player_profile: PlayerProfileLbType,
    limit: u16,
) -> Result<Leaderboard<LbPlayerProfile>, WynnApiError> {
    api_request(&format!(
        "{API_LOCATION}/leaderboards/{player_profile}?resultLimit={limit}"
    ))
    .await
}

pub async fn leaderboard_player_global(
    player_global: PlayerGlobalLbType,
    limit: u16,
) -> Result<Leaderboard<LbPlayerGlobal>, WynnApiError> {
    api_request(&format!(
        "{API_LOCATION}/leaderboards/{player_global}?resultLimit={limit}"
    ))
    .await
}

pub async fn leaderboard_raid_player(
    player_raid: RaidPlayerLbType,
    limit: u16,
) -> Result<Leaderboard<LbRaidPlayer>, WynnApiError> {
    api_request(&format!(
        "{API_LOCATION}/leaderboards/{player_raid}?resultLimit={limit}"
    ))
    .await
}

pub async fn leaderboard_raid_guild(
    guild_raid: RaidGuildLbType,
    limit: u16,
) -> Result<Leaderboard<LbRaidGuild>, WynnApiError> {
    api_request(&format!(
        "{API_LOCATION}/leaderboards/{guild_raid}?resultLimit={limit}"
    ))
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn guild_leaderboards() {
        for t in [
            GuildLbType::GuildLevel,
            GuildLbType::GuildTerritories,
            GuildLbType::GuildWars,
        ] {
            dbg!(t);
            let res = leaderboard_guild(t, 1).await;
            assert!(res.is_ok());
        }
    }

    #[tokio::test]
    async fn player_profile_leaderboards() {
        for t in [
            PlayerProfileLbType::WoodcuttingLevel,
            PlayerProfileLbType::MiningLevel,
            PlayerProfileLbType::FishingLevel,
            PlayerProfileLbType::FarmingLevel,
            PlayerProfileLbType::AlchemismLevel,
            PlayerProfileLbType::ArmouringLevel,
            PlayerProfileLbType::CookingLevel,
            PlayerProfileLbType::JewelingLevel,
            PlayerProfileLbType::ScribingLevel,
            PlayerProfileLbType::TailoringLevel,
            PlayerProfileLbType::WeaponsmithingLevel,
            PlayerProfileLbType::WoodworkingLevel,
            PlayerProfileLbType::PlayerContent,
            PlayerProfileLbType::CombatSoloLevel,
            PlayerProfileLbType::ProfessionsSoloLevel,
            PlayerProfileLbType::TotalSoloLevel,
            PlayerProfileLbType::HardcoreLegacyLevel,
            PlayerProfileLbType::IronmanContent,
            PlayerProfileLbType::UltimateIronmanContent,
            PlayerProfileLbType::HardcoreContent,
            PlayerProfileLbType::CraftsmanContent,
            PlayerProfileLbType::HuntedContent,
            PlayerProfileLbType::HuicContent,
            PlayerProfileLbType::HuichContent,
            PlayerProfileLbType::HichContent,
            PlayerProfileLbType::HicContent,
        ] {
            dbg!(t);
            let res = leaderboard_player_profile(t, 1).await;
            assert!(res.is_ok());
        }
    }

    #[tokio::test]
    async fn player_global_leaderboards() {
        for t in [
            PlayerGlobalLbType::ProfessionsGlobalLevel,
            PlayerGlobalLbType::CombatGlobalLevel,
            PlayerGlobalLbType::TotalGlobalLevel,
            PlayerGlobalLbType::GlobalPlayerContent,
            PlayerGlobalLbType::NogCompletion,
            PlayerGlobalLbType::TccCompletion,
            PlayerGlobalLbType::NolCompletion,
            PlayerGlobalLbType::WarsCompletion,
            PlayerGlobalLbType::TnaCompletion,
        ] {
            dbg!(t);
            let res = leaderboard_player_global(t, 1).await;
            assert!(res.is_ok());
        }
    }

    #[tokio::test]
    async fn raid_player_leaderboards() {
        for t in [
            RaidPlayerLbType::NogSrPlayers,
            RaidPlayerLbType::NolSrPlayers,
            RaidPlayerLbType::TccSrPlayers,
            RaidPlayerLbType::TnaSrPlayers,
        ] {
            dbg!(t);
            let res = leaderboard_raid_player(t, 1).await;
            assert!(res.is_ok());
        }
    }

    #[tokio::test]
    async fn raid_guild_leaderboards() {
        for t in [
            RaidGuildLbType::NogSrGuilds,
            RaidGuildLbType::NolSrGuilds,
            RaidGuildLbType::TccSrGuilds,
            RaidGuildLbType::TnaSrGuilds,
        ] {
            dbg!(t);
            let res = leaderboard_raid_guild(t, 1).await;
            assert!(res.is_ok());
        }
    }

    #[tokio::test]
    async fn longer_leaderboard() {
        let t = leaderboard_player_profile(PlayerProfileLbType::PlayerContent, 10).await;
        dbg!(&t);
        assert!(t.is_ok());
    }
}
