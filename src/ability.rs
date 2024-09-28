use crate::{
    api_request,
    classes::Class,
    deserialize_with_default,
    item::ItemRarity,
    leaderboard::deserialize_to_vec,
    player::{Ability, AbilityNodeCoordinate, Icon},
    Map, Set, WynnApiError, API_LOCATION,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct AbilityMap {
    #[serde(deserialize_with = "deserialize_to_vec")]
    pub pages: Vec<Vec<Ability>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AbilityTree {
    pub archetypes: Map<String, ArchetypeInfo>,
    #[serde(deserialize_with = "deserialize_to_vec")]
    pub pages: Vec<Map<String, AbilityInfo>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArchetypeInfo {
    pub name: String,
    pub description: String,
    pub short_description: String,
    pub icon: Icon,
    pub slot: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityInfo {
    pub name: String,
    pub icon: Icon,
    pub slot: u64,
    pub coordinates: AbilityNodeCoordinate,
    pub description: Vec<String>,
    pub requirements: AbilityRequirement,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub links: Set<String>,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub locks: Set<String>,
    pub page: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AbilityRequirement {
    pub ability_points: u8,
    pub node: Option<String>,
    pub archetype: Option<ArchetypeRequirements>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ArchetypeRequirements {
    pub name: String,
    pub amount: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Aspect {
    pub name: String,
    pub icon: Icon,
    /// only legendary+
    pub rarity: ItemRarity,
    pub required_class: Class,
    #[serde(deserialize_with = "deserialize_to_vec")]
    pub tiers: Vec<AspectTier>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AspectTier {
    pub threshold: u64,
    pub description: Vec<String>,
}

pub async fn abilitiy_map(class: Class) -> Result<AbilityMap, WynnApiError> {
    api_request(&format!(
        "{API_LOCATION}/ability/map/{}",
        class.main_class()
    ))
    .await
}

pub async fn abilitiy_tree(class: Class) -> Result<AbilityTree, WynnApiError> {
    api_request(&format!(
        "{API_LOCATION}/ability/tree/{}",
        class.main_class()
    ))
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn map() {
        let ability = abilitiy_map(Class::Warrior).await;
        assert!(ability.is_ok());
    }

    #[tokio::test]
    async fn tree() {
        let ability = abilitiy_tree(Class::Warrior).await;
        assert!(ability.is_ok());
    }

    #[tokio::test]
    async fn aspects() {
        let ability = abilitiy_tree(Class::Warrior).await;
        assert!(ability.is_ok());
    }
}
