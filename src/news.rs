use crate::{api_request, deserialize_from_string, WynnApiError, API_LOCATION};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewsArticle {
    pub title: String,
    pub date: String,
    pub forum_thread: String,
    pub author: String,
    pub content: String,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub comments: u64,
}

pub async fn latest_news() -> Result<Vec<NewsArticle>, WynnApiError> {
    api_request(&format!("{API_LOCATION}/latest-news")).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn news() {
        let news = latest_news().await;
        assert!(news.is_ok());
    }
}
