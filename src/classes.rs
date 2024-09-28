use std::fmt::Display;

use crate::{api_request, item::WeaponType, Map, WynnApiError, API_LOCATION};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Class {
    #[serde(alias = "ARCHER")]
    Archer,
    #[serde(alias = "HUNTER")]
    Hunter,
    #[serde(alias = "WARRIOR")]
    Warrior,
    #[serde(alias = "KNIGHT")]
    Knight,
    #[serde(alias = "MAGE")]
    Mage,
    #[serde(alias = "DARKWIZARD")]
    DarkWizard,
    #[serde(alias = "ASSASSIN")]
    Assassin,
    #[serde(alias = "NINJA")]
    Ninja,
    #[serde(alias = "SHAMAN")]
    Shaman,
    #[serde(alias = "SKYSEER")]
    Skyseer,
}

impl Class {
    pub const fn main_class(self) -> Self {
        use Class::*;
        match self {
            Hunter => Archer,
            Knight => Warrior,
            DarkWizard => Mage,
            Ninja => Assassin,
            Skyseer => Shaman,
            rest => rest,
        }
    }

    pub const fn donor_class(self) -> Self {
        use Class::*;
        match self {
            Archer => Hunter,
            Warrior => Knight,
            Mage => DarkWizard,
            Assassin => Ninja,
            Shaman => Skyseer,
            rest => rest,
        }
    }

    pub const fn weapon_type(self) -> WeaponType {
        use Class::*;
        match self {
            Archer | Hunter => WeaponType::Bow,
            Warrior | Knight => WeaponType::Spear,
            Mage | DarkWizard => WeaponType::Wand,
            Assassin | Ninja => WeaponType::Dagger,
            Shaman | Skyseer => WeaponType::Wand,
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = serde_json::to_string(&self)
            .map_err(|_| std::fmt::Error)?
            .trim_matches('"')
            .to_owned();
        f.write_str(&string)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassList {
    pub archer: ClassDifficulty,
    pub warrior: ClassDifficulty,
    pub assassin: ClassDifficulty,
    pub mage: ClassDifficulty,
    pub shaman: ClassDifficulty,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassDifficulty {
    pub name: String,
    pub overall_difficulty: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassInfo {
    pub id: String,
    pub name: String,
    pub lore: String,
    pub archetypes: Map<String, Archetype>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Archetype {
    pub name: String,
    pub difficulty: u64,
    pub damage: u64,
    pub defence: u64,
    pub range: u64,
    pub speed: u64,
}

pub async fn class_list() -> Result<ClassList, WynnApiError> {
    api_request(&format!("{API_LOCATION}/classes")).await
}

pub async fn class_info(class: Class) -> Result<ClassInfo, WynnApiError> {
    api_request(&format!("{API_LOCATION}/classes/{}", class.main_class())).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn classes() {
        let all = class_list().await;
        assert!(all.is_ok());
        for class in [
            Class::Archer,
            Class::Warrior,
            Class::Mage,
            Class::Assassin,
            Class::Shaman,
        ] {
            let info = class_info(class).await;
            assert!(info.is_ok());
        }
    }
}
