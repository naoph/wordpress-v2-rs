//! Unofficial library for the WordPress REST API

/// API interaction
pub mod api;

pub use api::Wordpress;

/// Types representing objects in the API
pub mod data;
