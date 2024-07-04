use crate::api::V1;
use crate::user::ApiUser;
use reqwest::{self, header::HeaderValue};

pub struct ArachnidShieldWithVersion<Version> {
    pub(crate) base_url: String,
    pub(crate) client: reqwest::blocking::Client,
    pub(crate) user: ApiUser,
    pub(crate) version: Version,
}

pub struct AsyncArachnidShieldWithVersion<Version> {
    pub(crate) base_url: String,
    pub(crate) client: reqwest::Client,
    pub(crate) user: ApiUser,
    pub(crate) version: Version,
}

// V1 is the default API.
pub type ArachnidShield = ArachnidShieldWithVersion<V1>;
pub type AsyncArachnidShield = AsyncArachnidShieldWithVersion<V1>;

pub(crate) static DEFAULT_BASE_URL: &str = "https://shield.projectarachnid.com";

impl ArachnidShieldWithVersion<V1> {
    /// Create a new ArachnidShield consumer
    /// given the API user.
    pub fn new(user: ApiUser) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(reqwest::header::AUTHORIZATION, user.to_header_value());

        let client = reqwest::blocking::ClientBuilder::default()
            .default_headers(headers)
            .build()
            .expect("the client should build with default authorization and accept headers.");

        Self {
            base_url: option_env!("ARACHNID_SHIELD_URL")
                .map(String::from)
                .unwrap_or(DEFAULT_BASE_URL.to_string()),
            client,
            user,
            version: V1,
        }
    }
}

impl AsyncArachnidShieldWithVersion<V1> {
    /// Create a new Async ArachnidShield consumer given the API user.
    pub fn new(user: ApiUser) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(reqwest::header::AUTHORIZATION, user.to_header_value());

        let client = reqwest::ClientBuilder::default()
            .default_headers(headers)
            .build()
            .expect("the client should build with default authorization and accept headers.");

        Self {
            base_url: option_env!("ARACHNID_SHIELD_URL")
                .map(String::from)
                .unwrap_or(DEFAULT_BASE_URL.to_string()),
            client,
            user,
            version: V1,
        }
    }
}
