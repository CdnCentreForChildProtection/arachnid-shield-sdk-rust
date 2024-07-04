use reqwest::header::HeaderValue;
use std::io::Write;

#[derive(Debug)]
pub struct ApiUser {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl ApiUser {
    /// Use this to instantiate an existing API user.
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub(crate) fn to_header_value(&self) -> reqwest::header::HeaderValue {
        let mut buf = b"Basic ".to_vec();
        {
            let mut encoder =
                base64::write::EncoderWriter::new(&mut buf, &base64::prelude::BASE64_STANDARD);
            _ = write!(encoder, "{}:", self.username);
            _ = write!(encoder, "{}", self.password);
        }
        let mut header =
            HeaderValue::from_bytes(&buf).expect("base64 should always be a valid HeaderValue");
        header.set_sensitive(true);
        header
    }
}
