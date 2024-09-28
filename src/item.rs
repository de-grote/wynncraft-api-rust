use crate::{
    api_request, classes::Class, deserialize_with_default, player::Icon, post_api_request, Map,
    Set, WynnApiError, API_LOCATION,
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemQuery {
    pub attack_speed: Set<AttackSpeed>,
    pub identifications: Set<Identification>,
    pub level_range: (u8, u8),
    pub major_ids: Set<MajorId>,
    pub professions: Set<Profession>,
    pub query: Option<String>,
    pub tier: Set<Rarity>,
    #[serde(rename = "type")]
    pub item_type: Set<ItemType>,
}

impl Default for ItemQuery {
    fn default() -> Self {
        Self {
            attack_speed: Default::default(),
            identifications: Default::default(),
            level_range: (0, 110),
            major_ids: Default::default(),
            professions: Default::default(),
            query: Default::default(),
            tier: Default::default(),
            item_type: Default::default(),
        }
    }
}

impl ItemQuery {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn set_query(&mut self, query: String) {
        self.query = Some(query);
    }

    #[inline]
    pub fn set_max_level(&mut self, max_lvl: u8) {
        self.level_range.1 = max_lvl;
    }

    #[inline]
    pub fn set_min_level(&mut self, min_lvl: u8) {
        self.level_range.0 = min_lvl;
    }

    #[inline]
    pub fn add_tier(&mut self, tier: impl Into<Rarity>) -> bool {
        self.tier.insert(tier.into())
    }

    #[inline]
    pub fn add_item_type(&mut self, item_type: impl Into<ItemType>) -> bool {
        self.item_type.insert(item_type.into())
    }

    #[inline]
    pub fn add_identification(&mut self, ident: impl Into<Identification>) -> bool {
        self.identifications.insert(ident.into())
    }

    #[inline]
    pub fn add_profession(&mut self, prof: impl Into<Profession>) -> bool {
        self.professions.insert(prof.into())
    }

    #[inline]
    pub fn add_major_id(&mut self, id: impl Into<MajorId>) -> bool {
        self.major_ids.insert(id.into())
    }

    #[inline]
    pub fn add_attack_speed(&mut self, speed: impl Into<AttackSpeed>) -> bool {
        self.attack_speed.insert(speed.into())
    }

    #[inline]
    pub fn with_query(query: impl Into<String>) -> Self {
        let mut s = Self::new();
        s.set_query(query.into());
        s
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemController {
    pub count: u64,
    pub pages: u64,
    pub previous: Option<u64>,
    pub current: u64,
    pub next: Option<u64>,
    pub links: LinksInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemResult {
    pub controller: ItemController,
    pub results: Map<String, Item>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinksInfo {
    pub previous: Option<String>,
    pub next: Option<String>,
}

// tool       ['gatheringSpeed', 'icon', 'identified', 'internalName', 'rarity', 'requirements', 'toolType', 'type']
//            ['gatheringSpeed', 'icon', 'identified', 'internalName', 'rarity', 'requirements', 'toolType', 'type']
// accessory  ['accessoryType', 'dropRestriction', 'icon', 'internalName', 'rarity', 'requirements', 'type']
//            ['accessoryType', 'base', 'dropMeta', 'dropRestriction', 'icon', 'identifications', 'identified', 'internalName', 'lore', 'majorIds', 'rarity', 'requirements', 'restrictions', 'type']
// tome       ['dropMeta', 'dropRestriction', 'icon', 'identifications', 'internalName', 'raidReward', 'rarity', 'requirements', 'restrictions', 'tomeType', 'type']
//            ['dropMeta', 'dropRestriction', 'icon', 'identifications', 'internalName', 'raidReward', 'rarity', 'requirements', 'restrictions', 'tomeType', 'type']
// weapon     ['attackSpeed', 'icon', 'internalName', 'rarity', 'requirements', 'type', 'weaponType']
//            ['allowCraftsman', 'attackSpeed', 'averageDps', 'base', 'dropMeta', 'dropRestriction', 'icon', 'identifications', 'identified', 'internalName', 'lore', 'majorIds', 'powderSlots', 'rarity', 'requirements', 'restrictions', 'type', 'weaponType']
// ingredient ['consumableOnlyIDs', 'icon', 'ingredientPositionModifiers', 'internalName', 'itemOnlyIDs', 'requirements', 'tier', 'type']
//            ['consumableOnlyIDs', 'droppedBy', 'icon', 'identifications', 'ingredientPositionModifiers', 'internalName', 'itemOnlyIDs', 'requirements', 'tier', 'type']
// charm      ['base', 'dropMeta', 'dropRestriction', 'icon', 'internalName', 'raidReward', 'rarity', 'requirements', 'restrictions', 'type']
//            ['base', 'dropMeta', 'dropRestriction', 'icon', 'identifications', 'internalName', 'raidReward', 'rarity', 'requirements', 'restrictions', 'type']
// armour     ['armourType', 'dropRestriction', 'internalName', 'rarity', 'requirements', 'type']
//            ['allowCraftsman', 'armourColor', 'armourMaterial', 'armourType', 'base', 'dropMeta', 'dropRestriction', 'icon', 'identifications', 'identified', 'internalName', 'lore', 'majorIds', 'powderSlots', 'rarity', 'requirements', 'restrictions', 'type']
// material   ['craftable', 'icon', 'identified', 'internalName', 'requirements', 'tier', 'type']
//            ['craftable', 'icon', 'identified', 'internalName', 'requirements', 'tier', 'type']
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub internal_name: String,
    /// can only be none if item type is armour
    pub icon: Option<Icon>,
    pub indentified: Option<bool>,
    #[serde(default)]
    pub drop_restriction: Option<DropRestriction>,
    pub lore: Option<String>,
    #[serde(flatten)]
    pub item_type: ItemTypeInfo,
}

impl Item {
    pub fn required_level(&self) -> u8 {
        match &self.item_type {
            ItemTypeInfo::Tool(x) => x.requirements.level,
            ItemTypeInfo::Accessory(x) => x.requirements.level,
            ItemTypeInfo::Tome(x) => x.requirements.level,
            ItemTypeInfo::Weapon(x) => x.requirements.level,
            ItemTypeInfo::Ingredient(x) => x.requirements.level,
            ItemTypeInfo::Charm(x) => x.requirements.level,
            ItemTypeInfo::Armour(x) => x.requirements.level,
            ItemTypeInfo::Material(x) => x.requirements.level,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ItemTypeInfo {
    Tool(ToolInfo),
    Accessory(AccessoryInfo),
    Tome(TomeInfo),
    Weapon(WeaponInfo),
    Ingredient(IngredientInfo),
    Charm(CharmInfo),
    Armour(ArmourInfo),
    Material(MaterialInfo),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MaterialInfo {
    pub requirements: LevelOnlyRequirements,
    pub tier: IngredientTier,
    pub identified: bool,
    pub craftable: Set<CraftedItemType>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArmourInfo {
    pub requirements: ItemRequirements,
    pub armour_type: ArmourType,
    pub rarity: ItemRarity,
    pub armour_material: Option<ArmourMaterial>,
    pub armour_color: Option<String>,
    pub drop_meta: Option<DropMeta>,
    pub restrictions: Option<Restrictions>,
    #[serde(default)]
    pub major_ids: Map<MajorId, String>,
    #[serde(default)]
    pub base: Map<Identification, IdentificationStats>,
    #[serde(default)]
    pub identifications: Map<Identification, IdentificationStats>,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub powder_slots: u8,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub allow_craftsman: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WeaponInfo {
    pub requirements: ItemRequirements,
    pub rarity: ItemRarity,
    pub weapon_type: WeaponType,
    pub attack_speed: AttackSpeed,
    #[serde(default)]
    pub base: Map<Identification, IdentificationStats>,
    #[serde(default)]
    pub identifications: Map<Identification, IdentificationStats>,
    #[serde(default)]
    pub major_ids: Map<MajorId, String>,
    pub drop_meta: Option<DropMeta>,
    pub restrictions: Option<Restrictions>,
    pub average_dps: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub powder_slots: u8,
    #[serde(default, deserialize_with = "deserialize_with_default")]
    pub allow_craftsman: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessoryInfo {
    pub requirements: ItemRequirements,
    pub rarity: ItemRarity,
    pub accessory_type: AccessoryType,
    #[serde(default)]
    pub base: Map<Identification, IdentificationStats>,
    #[serde(default)]
    pub identifications: Map<Identification, IdentificationStats>,
    pub restrictions: Option<Restrictions>,
    pub drop_meta: Option<DropMeta>,
    #[serde(default)]
    pub major_ids: Map<MajorId, String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolInfo {
    pub requirements: LevelOnlyRequirements,
    pub tool_type: ToolType,
    pub gathering_speed: u64,
    pub rarity: ItemRarity,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TomeInfo {
    pub requirements: LevelOnlyRequirements,
    pub rarity: ItemRarity,
    pub tome_type: TomeType,
    pub drop_meta: Option<DropMeta>,
    pub raid_reward: bool,
    pub restrictions: Option<Restrictions>,
    #[serde(default)]
    pub identifications: Map<Identification, IdentificationStats>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IngredientInfo {
    pub requirements: IngredientRequirements,
    pub tier: IngredientTier,
    #[serde(rename = "consumableOnlyIDs")]
    pub consumable_only_ids: ConsumableOnlyIds,
    #[serde(rename = "itemOnlyIDs")]
    pub item_only_ids: ItemOnlyIds,
    pub ingredient_position_modifiers: IngredientPositionModifiers,
    #[serde(default)]
    pub dropped_by: Vec<DropLocation>,
    #[serde(default)]
    pub identifications: Map<Identification, IdentificationStats>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharmInfo {
    pub requirements: CharmRequirements,
    pub rarity: ItemRarity,
    pub restrictions: Option<Restrictions>,
    pub drop_meta: Option<DropMeta>,
    pub raid_reward: bool,
    #[serde(default)]
    pub base: Map<Identification, IdentificationStats>,
    #[serde(default)]
    pub identifications: Map<Identification, IdentificationStats>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ArmourMaterial {
    Leather,
    Golden,
    Chain,
    Iron,
    Diamond,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemOnlyIds {
    #[serde(default)]
    pub durability_modifier: i64,
    #[serde(default)]
    pub strength_requirement: i64,
    #[serde(default)]
    pub dexterity_requirement: i64,
    #[serde(default)]
    pub intelligence_requirement: i64,
    #[serde(default)]
    pub defence_requirement: i64,
    #[serde(default)]
    pub agility_requirement: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IngredientPositionModifiers {
    #[serde(default)]
    pub left: i64,
    #[serde(default)]
    pub right: i64,
    #[serde(default)]
    pub above: i64,
    #[serde(default)]
    pub under: i64,
    #[serde(default)]
    pub touching: i64,
    #[serde(default)]
    pub not_touching: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum IdentificationStats {
    Static(i64),
    Dynamic(DynamicIdentificationStats),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicIdentificationStats {
    pub min: i64,
    pub raw: i64,
    pub max: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum DropRestriction {
    Never,
    Normal,
    Dungeon,
    Lootchest,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Restrictions {
    #[serde(rename = "quest item")]
    QuestItem,
    Soulbound,
    Untradable,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct ConsumableOnlyIds {
    #[serde(default)]
    pub duration: i64,
    #[serde(default)]
    pub charges: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DropMeta {
    pub name: String,
    /// has length of 3 or 4
    pub coordinates: Vec<i64>,
    #[serde(rename = "type")]
    pub drop_type: DropType,
    pub event: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DropLocation {
    pub name: String,
    pub location: Option<(i64, i64, i64, i64)>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DropType {
    Merchant,
    Lootrun,
    Raid,
    Miniboss,
    Guild,
    Altar,
    Quest,
    DungeonMerchant,
    Dungeon,
    Challenge,
    EventMerchant,
}

impl<'de> Deserialize<'de> for DropType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = serde_json::Value::deserialize(deserializer)?;
        Ok(match v {
            Value::String(s) => match s.as_ref() {
                "merchant" => DropType::Merchant,
                "lootrun" => DropType::Lootrun,
                "raid" => DropType::Raid,
                "miniboss" => DropType::Miniboss,
                "guild" => DropType::Guild,
                "altar" => DropType::Altar,
                "quest" => DropType::Quest,
                "dungeonMerchant" => DropType::DungeonMerchant,
                "dungeon" => DropType::Dungeon,
                "challenge" => DropType::Challenge,
                _ => Err(serde::de::Error::custom("not a dropType"))?,
            },
            Value::Array(_) => DropType::EventMerchant,
            _ => Err(serde::de::Error::custom("not a dropType"))?,
        })
    }
}

// tool ({'level'}, {'level'})
// tome ({'level'}, {'level'})
// material ({'level'}, {'level'})
// ingredient ({'level', 'skills'}, {'level', 'skills'})
// charm ({'levelRange', 'level'}, {'levelRange', 'level'})
// accessory ({'level'}, {'strength', 'dexterity', 'intelligence', 'level', 'classRequirement', 'agility', 'quest', 'defence'})
// armour ({'level'}, {'strength', 'dexterity', 'intelligence', 'level', 'classRequirement', 'agility', 'quest', 'defence'})
// weapon ({'level', 'classRequirement'}, {'strength', 'dexterity', 'intelligence', 'level', 'classRequirement', 'agility', 'quest', 'defence'})
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemRequirements {
    pub level: u8,
    pub strength: Option<u8>,
    pub dexterity: Option<u8>,
    pub intelligence: Option<u8>,
    pub defence: Option<u8>,
    pub agility: Option<u8>,
    pub quest: Option<String>,
    pub class_requirement: Option<Class>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IngredientRequirements {
    pub level: u8,
    pub skills: Set<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LevelOnlyRequirements {
    pub level: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharmRequirements {
    pub level: u8,
    pub level_range: LevelRange,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LevelRange {
    pub min: u8,
    pub max: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AttackSpeed {
    SuperSlow,
    VerySlow,
    Slow,
    Normal,
    Fast,
    VeryFast,
    SuperFast,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum CraftedItemType {
    Weapon(WeaponType),
    Armour(ArmourType),
    Accessory(AccessoryType),
    Other(OtherCraftedItemType),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum OtherCraftedItemType {
    #[serde(alias = "potions")]
    Potion,
    #[serde(alias = "scrolls")]
    Scroll,
    Food,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ItemType {
    Tool(ToolType),
    Weapon(WeaponType),
    Armour(ArmourType),
    Accessory(AccessoryType),
    Tome(TomeType),
    Other(OtherItemType),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum OtherItemType {
    Charm,
    Ingredient,
    Material,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ToolType {
    Axe,
    Rod,
    Pickaxe,
    Scythe,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum TomeType {
    #[serde(alias = "weapon_tome")]
    WeaponTome,
    #[serde(alias = "armour_tome")]
    ArmourTome,
    #[serde(alias = "guild_tome")]
    GuildTome,
    #[serde(alias = "expertise_tome")]
    ExpertiseTome,
    #[serde(alias = "mysticism_tome")]
    MysticismTome,
    #[serde(alias = "marathon_tome")]
    MarathonTome,
    #[serde(alias = "lootrun_tome")]
    LootrunTome,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum AccessoryType {
    #[serde(alias = "bracelets")]
    Bracelet,
    #[serde(alias = "necklaces")]
    Necklace,
    #[serde(alias = "rings")]
    Ring,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum WeaponType {
    #[serde(alias = "daggers")]
    Dagger,
    #[serde(alias = "bows")]
    Bow,
    #[serde(alias = "spears")]
    Spear,
    #[serde(alias = "reliks")]
    Relik,
    #[serde(alias = "wands")]
    Wand,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ArmourType {
    #[serde(alias = "helmets")]
    Helmet,
    #[serde(alias = "chestplates")]
    Chestplate,
    Leggings,
    Boots,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum Rarity {
    Item(ItemRarity),
    Ingredient(IngredientTier),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ItemRarity {
    Common,
    Unique,
    Rare,
    Legendary,
    Fabled,
    Set,
    Mythic,
}

#[derive(Serialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
pub enum IngredientTier {
    Ingredient0Star,
    Ingredient1Star,
    Ingredient2Star,
    Ingredient3Star,
}

impl<'de> Deserialize<'de> for IngredientTier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let err = Err(serde::de::Error::custom("Not a tier"));

        match value {
            Value::Number(num) => {
                if let Some(n) = num.as_u64() {
                    match n {
                        0 => Ok(IngredientTier::Ingredient0Star),
                        1 => Ok(IngredientTier::Ingredient1Star),
                        2 => Ok(IngredientTier::Ingredient2Star),
                        3 => Ok(IngredientTier::Ingredient3Star),
                        _ => err,
                    }
                } else {
                    err
                }
            }
            Value::String(s) => {
                if let Ok(n) = s.parse::<u8>() {
                    match n {
                        0 => Ok(IngredientTier::Ingredient0Star),
                        1 => Ok(IngredientTier::Ingredient1Star),
                        2 => Ok(IngredientTier::Ingredient2Star),
                        3 => Ok(IngredientTier::Ingredient3Star),
                        _ => err,
                    }
                } else {
                    err
                }
            }
            _ => err,
        }
    }
}

macro_rules! from_enum {
    ($from:ty, $to:ty, $name:ident) => {
        impl From<$from> for $to {
            fn from(val: $from) -> Self {
                <$to>::$name(val)
            }
        }
    };
}

from_enum!(IngredientTier, Rarity, Ingredient);
from_enum!(ItemRarity, Rarity, Item);
from_enum!(ToolType, ItemType, Tool);
from_enum!(WeaponType, ItemType, Weapon);
from_enum!(ArmourType, ItemType, Armour);
from_enum!(AccessoryType, ItemType, Accessory);
from_enum!(TomeType, ItemType, Tome);
from_enum!(OtherItemType, ItemType, Other);
from_enum!(WeaponType, CraftedItemType, Weapon);
from_enum!(ArmourType, CraftedItemType, Armour);
from_enum!(AccessoryType, CraftedItemType, Accessory);
from_enum!(OtherCraftedItemType, CraftedItemType, Other);

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Profession {
    Alchemism,
    Armouring,
    Cooking,
    Jeweling,
    Scribing,
    Tailoring,
    Weaponsmithing,
    Woodworking,
    Mining,
    Fishing,
    Farming,
    Woodcutting,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Identification {
    #[serde(rename = "rawStrength")]
    Strength,
    RawEarthDamage,
    BaseEarthDamage,
    #[serde(rename = "earthDamage")]
    EarthDamagePercent,
    EarthDefence,
    BaseEarthDefence,
    #[serde(rename = "earthSpellDamage")]
    EarthSpellDamagePercent,
    RawEarthSpellDamage,
    #[serde(rename = "earthMainAttackDamage")]
    EarthMainAttackDamagePercent,
    RawEarthMainAttackDamage,

    #[serde(rename = "rawDexterity")]
    Dexterity,
    RawThunderDamage,
    BaseThunderDamage,
    #[serde(rename = "thunderDamage")]
    ThunderDamagePercent,
    ThunderDefence,
    BaseThunderDefence,
    #[serde(rename = "thunderSpellDamage")]
    ThunderSpellDamagePercent,
    RawThunderSpellDamage,
    #[serde(rename = "thunderMainAttackDamage")]
    ThunderMainAttackDamagePercent,
    RawThunderMainAttackDamage,

    #[serde(rename = "rawIntelligence")]
    Intelligence,
    RawWaterDamage,
    BaseWaterDamage,
    #[serde(rename = "waterDamage")]
    WaterDamagePercent,
    WaterDefence,
    BaseWaterDefence,
    #[serde(rename = "waterSpellDamage")]
    WaterSpellDamagePercent,
    RawWaterSpellDamage,
    #[serde(rename = "waterMainAttackDamage")]
    WaterMainAttackDamagePercent,
    RawWaterMainAttackDamage,

    #[serde(rename = "rawDefence")]
    Defence,
    RawFireDamage,
    BaseFireDamage,
    #[serde(rename = "fireDamage")]
    FireDamagePercent,
    FireDefence,
    BaseFireDefence,
    #[serde(rename = "fireSpellDamage")]
    FireSpellDamagePercent,
    RawFireSpellDamage,
    #[serde(rename = "fireMainAttackDamage")]
    FireMainAttackDamagePercent,
    RawFireMainAttackDamage,

    #[serde(rename = "rawAgility")]
    Agility,
    RawAirDamage,
    BaseAirDamage,
    #[serde(rename = "airDamage")]
    AirDamagePercent,
    AirDefence,
    BaseAirDefence,
    #[serde(rename = "airSpellDamage")]
    AirSpellDamagePercent,
    RawAirSpellDamage,
    #[serde(rename = "airMainAttackDamage")]
    AirMainAttackDamagePercent,
    RawAirMainAttackDamage,

    #[serde(rename = "neutralDamage")]
    NeutralDamagePercent,
    #[serde(rename = "rawNeutralDamage")]
    RawNeutralDamage,
    #[serde(rename = "neutralMainAttackDamage")]
    NeutralMainAttackDamagePercent,
    #[serde(rename = "rawNeutralMainAttackDamage")]
    RawNeutralMainAttackDamage,
    #[serde(rename = "rawNeutralSpellDamage")]
    RawNeutralSpellDamage,

    #[serde(rename = "rawElementalSpellDamage")]
    RawElementalSpellDamage,
    #[serde(alias = "elementalDamageBonusRaw")]
    #[serde(rename = "ElementalDamageBonusRaw")]
    ElementalDamageBonusRaw,
    #[serde(rename = "elementalDamage")]
    ElementalDamagePercent,
    #[serde(rename = "rawElementalDamage")]
    RawElementalDamage,
    #[serde(rename = "elementalMainAttackDamage")]
    ElementalMainAttackDamagePercent,
    #[serde(rename = "rawElementalMainAttackDamage")]
    RawElementalMainAttackDamage,
    #[serde(rename = "elementalSpellDamage")]
    ElementalSpellDamagePercent,
    #[serde(rename = "elementalDefence")]
    ElementalDefence,

    #[serde(rename = "mainAttackDamage")]
    MainAttackDamagePercent,
    #[serde(rename = "rawAttackSpeed")]
    RawAttackSpeed,
    #[serde(rename = "rawMainAttackDamage")]
    RawMainAttackDamage,
    #[serde(rename = "poison")]
    Poison,
    #[serde(rename = "exploding")]
    Exploding,
    #[serde(rename = "baseDamage")]
    BaseDamage,
    #[serde(rename = "damage")]
    DamagePercent,
    #[serde(rename = "damageFromMobs")]
    DamageFromMobs,
    RawDamage,
    Knockback,

    #[serde(rename = "spellDamage")]
    SpellDamagePercent,
    #[serde(rename = "manaSteal")]
    ManaSteal,
    #[serde(rename = "manaRegen")]
    ManaRegen,
    #[serde(rename = "rawSpellDamage")]
    RawSpellDamage,
    #[serde(rename = "raw1stSpellCost")]
    RawFirstSpellCost,
    #[serde(rename = "raw2ndSpellCost")]
    RawSecondSpellCost,
    #[serde(rename = "raw3rdSpellCost")]
    RawThirdSpellCost,
    #[serde(rename = "raw4thSpellCost")]
    RawFourthSpellCost,
    #[serde(rename = "1stSpellCost")]
    FirstSpellCostPercent,
    #[serde(rename = "2ndSpellCost")]
    SecondSpellCostPercent,
    #[serde(rename = "3rdSpellCost")]
    ThirdSpellCostPercent,
    #[serde(rename = "4thSpellCost")]
    FourthSpellCostPercent,

    #[serde(rename = "healthRegenRaw")]
    HealthRegenRaw,
    #[serde(rename = "rawHealth")]
    Health,
    #[serde(rename = "lifeSteal")]
    LifeSteal,
    #[serde(rename = "healthRegen")]
    HealthRegenPercent,
    #[serde(rename = "baseHealth")]
    Basehealth,
    HealingEfficiency,
    Reflection,
    SlowEnemy,
    Thorns,
    WeakenEnemy,

    #[serde(rename = "walkSpeed")]
    WalkSpeed,
    #[serde(rename = "sprintRegen")]
    SprintRegen,
    #[serde(rename = "sprint")]
    Sprint,
    #[serde(rename = "jumpHeight")]
    JumpHeight,

    #[serde(rename = "lootBonus")]
    LootBonus,
    #[serde(rename = "xpBonus")]
    XpBonus,
    #[serde(rename = "leveledLootBonus")]
    LeveledLootBonus,
    #[serde(rename = "leveledXpBonus")]
    LeveledXpBonus,
    #[serde(rename = "stealing")]
    Stealing,
    #[serde(rename = "lootQuality")]
    LootQuality,
    #[serde(rename = "gatherXpBonus")]
    GatherXpBonus,
    #[serde(rename = "gatherSpeed")]
    GatherSpeed,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Hash)]
pub enum MajorId {
    #[serde(rename = "Alter Ego")]
    AlterEgo,
    #[serde(rename = "Better Coward Chant")]
    BetterCowardChant,
    #[serde(rename = "Better Lacerate")]
    BetterLacerate,
    #[serde(rename = "Blinding Lights")]
    BlindingLights,
    #[serde(rename = "Blood Flow")]
    BloodFlow,
    #[serde(rename = "Bomb Blast")]
    BombBlast,
    #[serde(rename = "Breeze Weaver")]
    BreezeWeaver,
    #[serde(rename = "Cavalryman")]
    Cavalryman,
    #[serde(rename = "Cherry Bombs")]
    CherryBombs,
    #[serde(rename = "Coagulate")]
    Coagulate,
    #[serde(rename = "Corporeal Shot")]
    CorporealShot,
    #[serde(rename = "Cryonic Cascade")]
    CryonicCascade,
    #[serde(rename = "Dead Weight")]
    DeadWeight,
    #[serde(rename = "Disabled Aerodynamics")]
    DisabledAerodynamics,
    #[serde(rename = "Disabled Leap")]
    DisabledLeap,
    #[serde(rename = "Disabled Mask Speed")]
    DisabledMaskSpeed,
    #[serde(rename = "Disabled Hop")]
    DisabledHop,
    #[serde(rename = "Disabled Righting Reflex")]
    DisabledRightingReflex,
    #[serde(rename = "Disabled Time Dilation")]
    DisabledTimeDilation,
    #[serde(rename = "Displace")]
    Displace,
    #[serde(rename = "Divine Honor")]
    DivineHonor,
    #[serde(rename = "Efflorescence")]
    Efflorescence,
    #[serde(rename = "Entropy")]
    Entropy,
    #[serde(rename = "Epileptic Motion")]
    EpilepticMotion,
    #[serde(rename = "Escape Route")]
    EscapeRoute,
    #[serde(rename = "Evershot")]
    Evershot,
    #[serde(rename = "Explosive Impact")]
    ExplosiveImpact,
    #[serde(rename = "Expunge")]
    Expunge,
    #[serde(rename = "Fallout")]
    Fallout,
    #[serde(rename = "Festive Spirit")]
    FestiveSpirit,
    #[serde(rename = "Find Thyself")]
    FindThyself,
    #[serde(rename = "Fission")]
    Fission,
    #[serde(rename = "Fixate")]
    Fixate,
    #[serde(rename = "Flashfreeze")]
    Flashfreeze,
    #[serde(rename = "Flurry of Blows")]
    FlurryOfBlows,
    #[serde(rename = "Forest's Blessing")]
    ForestsBlessing,
    #[serde(rename = "Freerunner")]
    Freerunner,
    #[serde(rename = "Frenetic Spirit")]
    FreneticSpirit,
    #[serde(rename = "Furious Effigy")]
    FuriousEffigy,
    #[serde(rename = "Furious Slices")]
    FuriousSlices,
    #[serde(rename = "Gentle Glow")]
    GentleGlow,
    #[serde(rename = "Geocentrism")]
    Geocentrism,
    #[serde(rename = "Gravity Well")]
    GravityWell,
    #[serde(rename = "Greed")]
    Greed,
    #[serde(rename = "Gruesome Knots")]
    GruesomeKnots,
    #[serde(rename = "Guardian")]
    Guardian,
    #[serde(rename = "Hawkeye")]
    Hawkeye,
    #[serde(rename = "Heart of the Pack")]
    HeartOfThePack,
    #[serde(rename = "Hellfire")]
    Hellfire,
    #[serde(rename = "Hurricane's Eye")]
    HurricanesEye,
    #[serde(rename = "Infernal Visage")]
    InfernalVisage,
    #[serde(rename = "Insoluble")]
    Insoluble,
    #[serde(rename = "Juggle")]
    Juggle,
    #[serde(rename = "Last Resort")]
    LastResort,
    #[serde(rename = "Lifestream")]
    Lifestream,
    #[serde(rename = "Lightweight")]
    Lightweight,
    #[serde(rename = "Lockdown")]
    Lockdown,
    #[serde(rename = "Lunge")]
    Lunge,
    #[serde(rename = "Lustrate")]
    Lustrate,
    #[serde(rename = "Madness")]
    Madness,
    #[serde(rename = "Magnet")]
    Magnet,
    #[serde(rename = "Mangle")]
    Mangle,
    #[serde(rename = "Meteor Crash")]
    MeteorCrash,
    #[serde(rename = "Mobility Reduction")]
    MobilityReduction,
    #[serde(rename = "Napalm")]
    Napalm,
    #[serde(rename = "Overwhelm")]
    Overwhelm,
    #[serde(rename = "Paragon")]
    Paragon,
    #[serde(rename = "Peaceful Effigy")]
    PeacefulEffigy,
    #[serde(rename = "Perfect Recall")]
    PerfectRecall,
    #[serde(rename = "Perilous Flare")]
    PerilousFlare,
    #[serde(rename = "Plague")]
    Plague,
    #[serde(rename = "Pounce")]
    Pounce,
    #[serde(rename = "Power Fist")]
    PowerFist,
    #[serde(rename = "Rally")]
    Rally,
    #[serde(rename = "Reckless Abandon")]
    RecklessAbandon,
    #[serde(rename = "Roving Assassin")]
    RovingAssassin,
    #[serde(rename = "Rock Shield")]
    RockShield,
    #[serde(rename = "Rusted Ichor")]
    RustedIchor,
    #[serde(rename = "Seekers' Volley")]
    SeekersVolley,
    #[serde(rename = "Saviour’s Sacrifice")]
    SavioursSacrifice,
    #[serde(rename = "Seeking Module")]
    SeekingModule,
    #[serde(rename = "Shamanic Influence")]
    ShamanicInfluence,
    #[serde(rename = "Snowy Steps")]
    SnowySteps,
    #[serde(rename = "Sorcery")]
    Sorcery,
    #[serde(rename = "Soul Eater")]
    SoulEater,
    #[serde(rename = "Strings of Fate")]
    StringsOfFate,
    #[serde(rename = "Sublimation")]
    Sublimation,
    #[serde(rename = "Tactical Brilliance")]
    TacticalBrilliance,
    #[serde(rename = "Tackle")]
    Tackle,
    #[serde(rename = "Taunt")]
    Taunt,
    #[serde(rename = "Tectonic Wrath")]
    TectonicWrath,
    #[serde(rename = "Temblor")]
    Temblor,
    #[serde(rename = "Tempest")]
    Tempest,
    #[serde(rename = "Totemic Fuse")]
    TotemicFuse,
    #[serde(rename = "Transcendence")]
    Transcendence,
    #[serde(rename = "Twisting Threads")]
    TwistingThreads,
    #[serde(rename = "Windsurf")]
    Windsurf,
}

pub async fn item_database(page: u64) -> Result<ItemResult, WynnApiError> {
    api_request(&format!("{API_LOCATION}/item/database?page={page}")).await
}

pub async fn item_database_full() -> Result<Map<String, Item>, WynnApiError> {
    api_request(&format!("{API_LOCATION}/item/database?fullResult")).await
}

pub async fn search_item(query: &ItemQuery) -> Result<ItemResult, WynnApiError> {
    post_api_request(&format!("{API_LOCATION}/item/search"), query).await
}

pub async fn search_item_full(query: &ItemQuery) -> Result<Map<String, Item>, WynnApiError> {
    post_api_request(&format!("{API_LOCATION}/item/search?fullResult"), query).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn items() {
        let items = item_database(143).await;
        assert!(items.is_ok());
    }

    #[tokio::test]
    async fn search() {
        let mut query = ItemQuery::with_query("photon");
        query.add_tier(ItemRarity::Unique);
        let item = search_item(&query).await;
        let item2 = search_item_full(&query).await;
        assert!(item.is_ok());
        assert!(item2.is_ok());
        let item = item.unwrap();
        if item.controller.next.is_none() {
            assert_eq!(item.results, item2.unwrap());
        }
    }
}
