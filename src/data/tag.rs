//! The `Tag` type and its related types

use serde::{Deserialize, Serialize};
use url::Url;

/// Represents a tag object
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Tag {
    pub id: usize,
    pub count: usize,
    pub description: String,
    pub link: Url,
    pub name: String,
    pub slug: String,
    pub taxonomy: TagTaxonomy,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TagTaxonomy {
    Category,
    PostTag,
    NavMenu,
    LinkCategory,
    PostFormat,
}
