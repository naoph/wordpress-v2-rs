//! The `Category` type and its related types

use serde::{Deserialize, Serialize};
use url::Url;

/// Represents a category object
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Category {
    pub id: usize,
    pub count: usize,
    pub description: String,
    pub link: Url,
    pub name: String,
    pub slug: String,
    pub taxonomy: CategoryTaxonomy,
    pub parent: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CategoryTaxonomy {
    Category,
    PostTag,
    NavMenu,
    LinkCategory,
    PostFormat,
}
