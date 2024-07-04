mod api;
mod client;
mod data_structures;
mod user;

pub use api::V1;
pub use data_structures::{
    ArachnidShieldError, ErrorDetail, MatchDetails, MatchType, MediaClassification,
    NearMatchDetails, ScanMediaFromBytes, ScanMediaFromUrl, ScanPdqHashes, ScannedMedia,
    ScannedPdqHashes,
};

pub use client::{
    ArachnidShield, ArachnidShieldWithVersion, AsyncArachnidShield, AsyncArachnidShieldWithVersion,
};

pub use user::ApiUser;
