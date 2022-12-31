use chrono::{DateTime, Utc};
use reqwest::{Client, RequestBuilder, Response};
use serde::Deserialize;
use snafu::{ResultExt, Snafu};
use url::Url;

macro_rules! add_query_arg {
    ($arg:ident, $type:ty) => {
        pub fn $arg(mut self, $arg: $type) -> Self {
            self.req = self.req.query(&[(stringify!($arg), $arg)]);
            self
        }
    };
}

macro_rules! add_send {
    ($type:ty) => {
        pub async fn send(self) -> Result<$type, RequestError> {
            let resp: Response = self.req
                .send()
                .await
                .context(HttpSnafu)?;

            let json: serde_json::Value = resp.json()
                .await
                .context(JsonSnafu)?;

            if let Ok(r) = serde_json::from_value::<$type>(json.clone()) {
                Ok(r)
            } else if let Ok(e) = serde_json::from_value::<WordpressError>(json) {
                Err(RequestError::WordpressError { source: e })
            } else {
                Err(RequestError::UnparsabeJsonError)
            }
        }
    };
}

fn graft_path<'a>(url: &Url, path: &'a str) -> Url {
    let mut url = url.clone();
    let sep = if url.path().ends_with("/") || path.starts_with("/") { "" } else { "/" };
    let new_path = format!("{}{}{}", url.path(), sep, path);
    url.set_path(&new_path);
    url
}

pub struct ListPostsRequest {
    pub req: RequestBuilder,
}

impl ListPostsRequest {
    // TODO: context
    add_query_arg!(page, usize);
    add_query_arg!(per_page, usize);
    add_query_arg!(search, &str);
    add_query_arg!(after, DateTime<Utc>);
    add_query_arg!(author, usize);
    // TODO: author_exclude
    add_query_arg!(before, DateTime<Utc>);
    // TODO: exclude
    // TODO: include
    add_query_arg!(offset, usize);
    // TODO: order
    // TODO: orderby
    // TODO: slug
    // TODO: status
    // TODO: tax_relation
    // TODO: categories
    // TODO: categories_exclude
    // TODO: tags
    // TODO: tags_exclude
    add_query_arg!(sticky, bool);

    add_send!(Vec<crate::data::Post>);
}

pub struct ListTagsRequest {
    pub req: RequestBuilder,
}

impl ListTagsRequest {
    // TODO: context
    add_query_arg!(page, usize);
    add_query_arg!(per_page, usize);
    add_query_arg!(search, String);
    // TODO: exclude
    // TODO: include
    add_query_arg!(offset, usize);
    // TODO: order
    // TODO: orderby
    add_query_arg!(hide_empty, bool);
    // TODO: post
    // TODO: slug

    add_send!(Vec<crate::data::Tag>);
}

/// An interface to a WordPress install
pub struct Wordpress {
    client: Client,
    location: Url,
}

impl Wordpress {
    /// Create an interface to a specified WordPress install
    pub fn new(location: Url) -> Wordpress {
        let ua = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        let client = Client::builder()
            .user_agent(ua)
            .build()
            .unwrap();

        Wordpress {
            client,
            location,
        }
    }

    /// Retrieve a list of post objects
    pub fn list_posts(&self) -> ListPostsRequest {
        let target = graft_path(&self.location, "wp-json/wp/v2/posts");
        ListPostsRequest { req: self.client.get(target) }
    }

    /// Retrieve a list of tag objects
    pub fn list_tags(&self) -> ListTagsRequest {
        let target = graft_path(&self.location, "wp-json/wp/v2/tags");
        ListTagsRequest { req: self.client.get(target) }
    }
}


/// Error potentially returned by WordPress instead of the expected content
#[derive(Debug, Deserialize)]
pub struct WordpressError {
    pub code: String,
    pub message: String,
    pub data: WordpressErrorData,
}

impl std::fmt::Display for WordpressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for WordpressError {}

#[derive(Debug, Deserialize)]
pub struct WordpressErrorData {
    pub status: usize,
}

/// Potential errors when executing a generated request
#[derive(Debug, Snafu)]
pub enum RequestError {
    /// Failed to retrieve a response from the server 
    HttpError { source: reqwest::Error },

    /// Response did not contain valid JSON data
    JsonError { source: reqwest::Error },

    /// Received an error response from the server
    WordpressError { source: WordpressError },

    /// Received valid JSON representing neither the requested data nor a valid error
    UnparsabeJsonError,
}
