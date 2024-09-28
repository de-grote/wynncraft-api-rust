use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::{fmt::Display, str::FromStr, sync::OnceLock};
use thiserror::Error;

pub mod ability;
pub mod classes;
pub mod guild;
pub mod item;
pub mod leaderboard;
pub mod map;
pub mod news;
pub mod player;
pub mod search;

#[cfg(not(feature = "BTree"))]
type Map<K, V> = HashMap<K, V>;
#[cfg(not(feature = "BTree"))]
type Set<K> = HashSet<K>;

#[cfg(feature = "BTree")]
type Map<K, V> = BTreeMap<K, V>;
#[cfg(feature = "BTree")]
type Set<K> = BTreeSet<K>;

pub const API_LOCATION: &str = "https://api.wynncraft.com/v3";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Identifier {
    Username,
    Uuid,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Identifier::Username => "username",
            Identifier::Uuid => "uuid",
        })
    }
}

#[derive(Debug, Error)]
pub enum WynnApiError {
    #[error("couldn't connect to the api")]
    ConnectionError(#[from] reqwest::Error),
    #[error("there are multiple choices of what the api can return")]
    MultipleChoices(Map<String, Map<String, serde_json::Value>>),
    #[cfg(feature = "no_panic")]
    #[error("this error should never occur")]
    DeserializeError(#[from] serde_json::Error),
}

static CONNECTION_CLIENT: OnceLock<Client> = OnceLock::new();

async fn api_request<T>(link: &str) -> Result<T, WynnApiError>
where
    T: DeserializeOwned,
{
    let client = CONNECTION_CLIENT.get_or_init(Client::new);
    let response = client.get(link).send().await?;
    let response = response.error_for_status()?;
    let multi_selector = response.status().is_redirection();
    let text = response.text().await?;
    deserialize_data(link, &text, multi_selector)
}

async fn post_api_request<T, Q>(link: &str, content: Q) -> Result<T, WynnApiError>
where
    T: DeserializeOwned,
    Q: Serialize,
{
    let client = CONNECTION_CLIENT.get_or_init(Client::new);
    let body = serde_json::to_string(&content).expect("valid serializable data");
    let response = client
        .post(link)
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let response = response.error_for_status()?;
    let multi_selector = response.status().is_redirection();
    let text = response.text().await?;
    deserialize_data(link, &text, multi_selector)
}

fn deserialize_data<T>(link: &str, text: &str, multi_selector: bool) -> Result<T, WynnApiError>
where
    T: DeserializeOwned,
{
    if multi_selector {
        return Err(WynnApiError::MultipleChoices(
            match serde_json::from_str(text) {
                Ok(x) => x,
                Err(e) => parsing_error(link, e, text)?,
            },
        ));
    }
    let stats = match serde_json::from_str::<T>(text) {
        Ok(x) => x,
        Err(e) => parsing_error(link, e, text)?,
    };
    Ok(stats)
}

fn parsing_error<T>(link: &str, error: serde_json::Error, text: &str) -> Result<T, WynnApiError> {
    #[cfg(not(feature = "no_panic"))]
    // this is only truly unreachable in my perfect world where everything works
    unreachable!(
        "wrapper is not working on request: `{link}`\n{error}\n{}",
        if text.len() < 2000 {
            text
        } else {
            let place = error.column();
            &text[(place.saturating_sub(200))..(place + 200).min(text.len())]
        }
    );
    #[cfg(feature = "no_panic")]
    Err(WynnApiError::DeserializeError(error))
}

fn deserialize_with_default<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Default,
    D: Deserializer<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

fn deserialize_from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    s.parse::<T>().map_err(serde::de::Error::custom)
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub struct World(pub u8);

impl Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;

        if let Some(number) = s.strip_prefix("WC") {
            number
                .parse::<u8>()
                .map(World)
                .map_err(serde::de::Error::custom)
        } else {
            Err(serde::de::Error::custom("String is not a wynncraft world"))
        }
    }
}

impl ToString for World {
    fn to_string(&self) -> String {
        format!("WC{}", self.0)
    }
}

impl World {
    #[inline]
    pub const fn new(world_number: u8) -> Self {
        Self(world_number)
    }

    #[inline]
    pub const fn world_number(&self) -> u8 {
        self.0
    }
}
