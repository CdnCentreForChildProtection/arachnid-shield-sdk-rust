use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A collection of errors that can occur when interacting
/// with the Arachnid Shield API.
#[derive(thiserror::Error, Debug)]
pub enum ArachnidShieldError {
    /// Represents any failed but complete interaction with the Arachnid Shield API.
    /// This usually indicates either the provided media was unsupported,
    /// or the API user credentials could not be validated.
    ///
    /// To set up an account, visit [Arachnid Shield]'s [contact us] page and
    /// get in touch with us.
    ///
    /// [Arachnid Shield]: https://projectarachnid.ca/en/#shield
    /// [contact us]: https://projectarachnid.ca/en/contact/
    #[error("Arachnid Shield API request failed: {0}")]
    APIError(ErrorDetail),
    /// Represents anything that could go wrong while initiating
    /// a request to the Arachnid Shield API.
    #[error(transparent)]
    ReqwestFailed(#[from] reqwest::Error),
    /// Represents the mime type we intend to send to Arachnid Shield
    /// could not be sent because it is not a valid http header.
    #[error("Provided mime type: {0} is not a valid http header")]
    BadMimeType(String),
    /// Represents anything that can go wrong while reading media
    /// from a file.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    /// Represents that the mime type could not be recognized
    /// when processing a file.
    #[error("Could not identify mime type for file: {0}")]
    FailedToRecognizeMimeType(String),
}

/// A match object representing the image in our database that has the
/// same cryptographic hash as the scanned image.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MatchDetails {
    /// The base32 representation of the SHA1 hash of the media.
    pub sha1_base32: String,
    /// The hexadecimal representation of the SHA256 hash of the media.
    pub sha256_hex: String,
    /// The classification category for this media, if any.
    pub classification: Option<MediaClassification>,
    /// The numeric distance between the two images.
    /// A distance below 5000 represents a close match; below 2000 is very close.
    pub distance: usize,
}

/// A record of a near match (based on perceptual hashing) to a known image in our database.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NearMatchDetails {
    /// The time, in seconds, in the submitted video file where the match was found.
    /// For still images this will be 0.
    pub timestamp: f64,
    /// The base-32 representation of the SHA1 cryptographic hash of the media in our database.
    pub sha1_base32: String,
    /// The base-16 (hexadecimal) representation of the SHA256 cryptographic hash of the media in our database.
    pub sha256_hex: String,
    /// The classification of the media in our database.
    pub classification: Option<MediaClassification>,
}

/// A record of a media (+ metadata) that has been scanned by the Arachnid Shield API
/// and potential any visual or cryptographic matches attached to it.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ScannedMedia {
    /// The base-32 representation of the SHA1 cryptographic hash of the media.
    pub sha1_base32: String,
    /// The base-16 (hexadecimal) representation of the SHA256 cryptographic hash of the media.
    pub sha256_hex: String,
    /// The total size, in bytes, of the media that was scanned.
    pub size_bytes: usize,
    /// The classification assigned to this media.
    pub classification: Option<MediaClassification>,
    /// The technology that was used to verify a match between two media.
    pub match_type: Option<MatchType>,
    /// A record of a near match (based on perceptual hashing) to a known image in our database.
    pub near_match_details: Vec<NearMatchDetails>,
}

impl ScannedMedia {
    /// Determine whether the scanned media has known matches.
    #[inline(always)]
    pub fn no_known_match(&self) -> bool {
        matches!(self.classification, Some(MediaClassification::NoKnownMatch))
    }

    /// Determine whether the scanned media matches any known media.
    #[inline(always)]
    pub fn matches_known_media(&self) -> bool {
        self.classification
            .is_some_and(|classification| classification != MediaClassification::NoKnownMatch)
    }
}

/// A record of a match for PDQ hash that has been scanned by the Arachnid Shield API
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ScannedPdqHash {
    /// The classification assigned to this media.
    pub classification: MediaClassification,
    /// The technology that was used to verify a match between two media.
    pub match_type: Option<MatchType>,
    /// A record of a near match (based on perceptual hashing) to a known image in our database.
    pub near_match_details: Option<NearMatchDetails>,
}

impl ScannedPdqHash {
    /// Determine whether the scanned PDQ hash matches any known media.
    #[inline(always)]
    pub fn matches_known_media(&self) -> bool {
        self.classification != MediaClassification::NoKnownMatch
    }
}

/// A record of the matches for PDQ hashes that have been scanned by the Arachnid Shield API.
#[derive(Deserialize, Serialize, Debug)]
pub struct ScannedPdqHashes {
    /// A collection of the match details for scanned PDQ hashes.
    pub scanned_hashes: HashMap<String, ScannedPdqHash>,
}

/// A disjoint union of categories that a media
/// could be classified as.
/// A list of the possible categories that an image or video could be classified as.
///
/// ### Note
///
/// Video files are classified based on their frames. So, if any frame from a video matches a known `csam` image, the video will be classified as `csam`.
/// Similarly, if any frame matches a `harmful-abusive-material` image, the video will be classified as `harmful-abusive-material`.
/// If both `csam` and `harmful-abusive-material` frames are matched in a single video, the classification `csam` will be returned.
///
/// More classification types may be added in the future.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum MediaClassification {
    /// Child sexual abuse material, also known as "child pornography".
    #[serde(rename = "csam")]
    CSAM,
    /// Content considered harmful to children includes all images or videos associated with the abusive incident, nude or partially nude images or videos of children that have become publicly available and are used in a sexualized context or connected to sexual commentary.
    HarmfulAbusiveMaterial,
    /// The media was not an exact match or near match to any classified CSAM or harmful/abusive material in our database.
    NoKnownMatch,
}

/// The technology that was used to verify a match between two media.
/// This indicates whether the submitted media matched media in our database exactly (by cryptographic hash) or visually (by visual hash).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum MatchType {
    /// An exact cryptographic hash match using SHA1
    Exact,
    /// A visual near-match using PhotoDNA
    Near,
}

/// A representation of a request to scan media from a url.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScanMediaFromUrl {
    /// The url to get the media from.
    url: String,
}

impl ScanMediaFromUrl {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub(crate) fn body(&self) -> Vec<u8> {
        serde_json::to_vec(&self).expect("Should be able to serialize a ScanMediaFromUrl request.")
    }
}

/// A representation of a request to scan media from provided bytes.
#[derive(Debug)]
pub struct ScanMediaFromBytes {
    /// The raw contents of a media in bytes.
    data: Vec<u8>,
    /// The mime type for the given media.
    mime_type: mime::Mime,
}

impl ScanMediaFromBytes {
    /// Get the mime type as a string.
    pub fn mime_type(&self) -> String {
        self.mime_type.to_string()
    }
    /// Build a new request to scan media from provided bytes.
    pub fn new(data: impl Into<Vec<u8>>, mime_type: mime::Mime) -> Self {
        Self {
            data: data.into(),
            mime_type,
        }
    }
    /// Consume the stored data in this request container.
    pub(crate) fn body(self) -> Vec<u8> {
        self.data
    }
}

/// A representation of a request to scan a pdq list.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScanPdqHashes {
    hashes: Vec<String>, // List of base64 encoded hashes
}

impl ScanPdqHashes {
    /// Each pdq is bytes at input
    pub fn new(data: &[[u8; 32]]) -> Self {
        // Firstly, encode the bytes to base64 string
        Self {
            hashes: data.iter().map(|x| BASE64_STANDARD.encode(x)).collect(),
        }
    }

    // JSON object
    pub(crate) fn body(&self) -> Vec<u8> {
        serde_json::to_vec(&self).expect("Should be able to serialize a ScanPdqHashes request.")
    }
}

/// A container for any error messages that
/// the Arachnid Shield API sends us.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    /// The actual error message inside the container.
    pub detail: String,
}

impl AsRef<str> for ErrorDetail {
    fn as_ref(&self) -> &str {
        self.detail.as_ref()
    }
}

impl core::fmt::Display for ErrorDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.detail)
    }
}
