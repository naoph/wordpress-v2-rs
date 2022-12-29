//! The `Post` type and its related types

use chrono::{NaiveDateTime, DateTime, Utc};
use serde::{Deserialize, Serialize, de};
use url::Url;

fn deserialize_utc<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let naive: NaiveDateTime = de::Deserialize::deserialize(deserializer)?;
    Ok(DateTime::<Utc>::from_utc(naive, Utc))
}

fn deserialize_opt_utc<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let naive: Option<NaiveDateTime> = de::Deserialize::deserialize(deserializer)?;
    match naive {
        None => Ok(None),
        Some(n) => Ok(Some(DateTime::<Utc>::from_utc(n, Utc))),
    }
}

/// Represents a post object
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Post {
    pub date: Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_opt_utc")]
    pub date_gmt: Option<DateTime<Utc>>,
    pub guid: Guid,
    pub id: isize,
    pub link: Url,
    pub modified: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_utc")]
    pub modified_gmt: DateTime<Utc>,
    pub slug: String,
    pub status: Status,
    pub r#type: String,
    pub title: Title,
    pub content: Content,
    pub author: isize,
    pub excerpt: Excerpt,
    pub featured_media: isize,
    pub comment_status: CommentStatus,
    pub ping_status: PingStatus,
    pub format: Format,
    pub sticky: bool,
    pub template: String,
    pub categories: Vec<isize>,
    pub tags: Vec<isize>,
}

/// Represents the object in a post's `guid` field
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Guid {
    pub rendered: String,
}

/// Represents a post's `status` field
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Publish,
    Future,
    Draft,
    Pending,
    Private,
}

/// Represents the object in a post's `title` field
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Title {
    pub rendered: String,
}

/// Represents the object in a post's `content` field
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Content {
    pub rendered: String,
    pub protected: bool,
}

/// Represents the object in a post's `excerpt` field
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Excerpt {
    pub rendered: String,
    pub protected: bool,
}

/// Represents a post's `comment_status` field
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentStatus {
    Open,
    Closed,
}

/// Represents a post's `ping_status` field
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PingStatus {
    Open,
    Closed,
}

/// Represents a post's `format` field
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Standard,
    Aside,
    Chat,
    Gallery,
    Link,
    Image,
    Quote,
    Status,
    Video,
    Audio,
}
