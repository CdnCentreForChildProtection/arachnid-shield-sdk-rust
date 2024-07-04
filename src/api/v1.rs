use crate::client::{ArachnidShieldWithVersion, AsyncArachnidShieldWithVersion};
use crate::data_structures::{
    ArachnidShieldError, ErrorDetail, ScanMediaFromBytes, ScanMediaFromUrl, ScanPdqHashes,
    ScannedMedia, ScannedPdqHashes,
};
use std::io::Read;

/// A marker struct that defines a public api
/// for V1 of the Arachnid Shield API.
#[derive(Debug, Default)]
pub struct V1;

impl core::fmt::Debug for ArachnidShieldWithVersion<V1> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArachnidShield")
            .field("api_version", &self.version)
            .field("user", &self.user.username)
            .finish()
    }
}

impl core::fmt::Debug for AsyncArachnidShieldWithVersion<V1> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncArachnidShield")
            .field("api_version", &self.version)
            .field("user", &self.user.username)
            .finish()
    }
}

impl ArachnidShieldWithVersion<V1> {
    /// Build an endpoint, given the path.
    fn endpoint(&self, path: &str) -> Result<url::Url, ArachnidShieldError> {
        let root_url =
            url::Url::parse(&self.base_url).expect("Should be able to parse the base url");
        Ok(root_url
            .join(&format!("/v1/{path}/"))
            .expect("Should be able to join path to base url."))
    }

    /// Given the contents of some media, and a mime type for it,
    /// scan the contents for CSAM using the Arachnid Shield API.
    ///
    /// ## Examples
    /// ```no_run
    /// use arachnid_shield::{ArachnidShield, ApiUser};
    ///
    /// fn get_media() -> Vec<u8> {
    ///     return vec![]
    /// }
    ///
    /// let client = ArachnidShield::new(
    ///     ApiUser::new("<username>", "<password>")
    /// );
    ///
    /// // Suppose you have media contents and mime_type already available.
    /// let contents = get_media();
    /// let mime_type = mime::IMAGE_JPEG;
    ///
    /// // Request Arachnid Shield to scan the media.
    /// let response = client.scan_media_from_bytes(contents, mime_type);
    ///
    /// // Might want to handle errors in practice, but we'll
    /// // just .unwrap() it here in this example.
    /// let scanned_media = response.unwrap();
    ///
    /// if scanned_media.matches_known_media() {
    ///     eprintln!("Uh-oh, this media: {:#?} matches known media.", scanned_media);
    /// }
    /// ```
    pub fn scan_media_from_bytes<T: Into<Vec<u8>>>(
        &self,
        data: T,
        mime_type: mime::Mime,
    ) -> Result<ScannedMedia, ArachnidShieldError> {
        self.scan_media_from_bytes_with_config(ScanMediaFromBytes::new(data, mime_type))
    }

    /// Given a path to a file, scan its contents for CSAM using the Arachnid Shield API.
    ///
    /// ## Examples
    /// ```no_run
    /// use arachnid_shield::{ArachnidShield, ApiUser};
    ///
    /// let client = ArachnidShield::new(
    ///     ApiUser::new("<username>", "<password>")
    /// );
    ///
    /// let filepath = "../myimg.png";
    ///
    /// let scanned_media = client.scan_media_from_file(filepath).unwrap();
    ///
    /// if scanned_media.matches_known_media() {
    ///     eprintln!("Uh-oh, this media: {:#?} matches known media.", scanned_media);
    /// }
    ///
    /// ```
    pub fn scan_media_from_file<P: AsRef<std::path::Path>>(
        &self,
        filepath: P,
    ) -> Result<ScannedMedia, ArachnidShieldError> {
        let mut file = std::fs::File::open(filepath.as_ref())?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let filepath_as_str = filepath.as_ref().to_string_lossy().to_string();

        // Guess a mime type for the media.
        let mime_type = mime_guess::from_path(filepath).first().ok_or(
            ArachnidShieldError::FailedToRecognizeMimeType(filepath_as_str),
        )?;

        self.scan_media_from_bytes_with_config(ScanMediaFromBytes::new(contents, mime_type))
    }

    /// Given a `ScanMediaFromBytes` request, send it to
    /// the Arachnid Shield API.
    ///
    pub fn scan_media_from_bytes_with_config(
        &self,
        params: ScanMediaFromBytes,
    ) -> Result<ScannedMedia, ArachnidShieldError> {
        let response = self
            .client
            .post(self.endpoint("media")?)
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_str(&params.mime_type())
                    .map_err(|_| ArachnidShieldError::BadMimeType(params.mime_type()))?,
            )
            .body(params.body())
            .send()?;
        if response.status().is_success() {
            response
                .json::<ScannedMedia>()
                .map_err(ArachnidShieldError::ReqwestFailed)
        } else {
            Err(ArachnidShieldError::APIError(
                response
                    .json::<ErrorDetail>()
                    .expect("error should be parsed as an ErrorDetail"),
            ))
        }
    }

    pub fn download_and_scan_media_from_url(
        &self,
        url: &str,
    ) -> Result<ScannedMedia, ArachnidShieldError> {
        self.download_and_scan_media_from_url_with_config(ScanMediaFromUrl::new(url))
    }

    /// Given a `ScanMediaFromUrl` request, send it to
    /// the Arachnid Shield API.
    ///
    pub fn download_and_scan_media_from_url_with_config(
        &self,
        params: ScanMediaFromUrl,
    ) -> Result<ScannedMedia, ArachnidShieldError> {
        let response = self
            .client
            .post(self.endpoint("url")?)
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .body(params.body())
            .send()?;

        if response.status().is_success() {
            response
                .json::<ScannedMedia>()
                .map_err(ArachnidShieldError::ReqwestFailed)
        } else {
            Err(ArachnidShieldError::APIError(
                response
                    .json::<ErrorDetail>()
                    .expect("error should be parsed as an ErrorDetail"),
            ))
        }
    }

    /// Given the PDQ of some media, scan the contents for CSAM using the Arachnid Shield API.
    pub fn scan_pdq_hashes(
        &self,
        data: &[[u8; 32]],
    ) -> Result<ScannedPdqHashes, ArachnidShieldError> {
        self.scan_pdq_hashes_with_config(ScanPdqHashes::new(data))
    }

    /// Given a 'ScanPdqHashes' request, send it to the Arachnid Shield API.
    pub fn scan_pdq_hashes_with_config(
        &self,
        params: ScanPdqHashes,
    ) -> Result<ScannedPdqHashes, ArachnidShieldError> {
        let response = self
            .client
            .post(self.endpoint("pdq")?)
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .body(params.body())
            .send()?;

        if response.status().is_success() {
            response
                .json::<ScannedPdqHashes>()
                .map_err(ArachnidShieldError::ReqwestFailed)
        } else {
            Err(ArachnidShieldError::APIError(
                response
                    .json::<ErrorDetail>()
                    .expect("error should be parsed as an ErrorDetail"),
            ))
        }
    }
}

impl AsyncArachnidShieldWithVersion<V1> {
    /// Build an endpoint, given the path.
    fn endpoint(&self, path: &str) -> Result<url::Url, ArachnidShieldError> {
        let root_url =
            url::Url::parse(&self.base_url).expect("Should be able to parse the base url");
        Ok(root_url
            .join(&format!("/v1/{path}/"))
            .expect("Should be able to join path to base url."))
    }

    /// Given the contents of some media, and a mime type for it, scan the contents for CSAM using the Arachnid Shield API.
    pub async fn scan_media_from_bytes<T: Into<Vec<u8>>>(
        &self,
        data: T,
        mime_type: mime::Mime,
    ) -> Result<ScannedMedia, ArachnidShieldError> {
        self.scan_media_from_bytes_with_config(ScanMediaFromBytes::new(data, mime_type))
            .await
    }

    /// Given a path to a file, scan its contents for CSAM using the Arachnid Shield API.
    pub async fn scan_media_from_file<P: AsRef<std::path::Path>>(
        &self,
        filepath: P,
    ) -> Result<ScannedMedia, ArachnidShieldError> {
        let mut file = std::fs::File::open(filepath.as_ref())?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let filepath_as_str = filepath.as_ref().to_string_lossy().to_string();

        // Guess a mime type for the media.
        let mime_type = mime_guess::from_path(filepath).first().ok_or(
            ArachnidShieldError::FailedToRecognizeMimeType(filepath_as_str),
        )?;

        self.scan_media_from_bytes_with_config(ScanMediaFromBytes::new(contents, mime_type))
            .await
    }

    /// Given a `ScanMediaFromBytes` request, send it to the Arachnid Shield API.
    pub async fn scan_media_from_bytes_with_config(
        &self,
        params: ScanMediaFromBytes,
    ) -> Result<ScannedMedia, ArachnidShieldError> {
        let response = self
            .client
            .post(self.endpoint("media")?)
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_str(&params.mime_type())
                    .map_err(|_| ArachnidShieldError::BadMimeType(params.mime_type()))?,
            )
            .body(params.body())
            .send()
            .await?;
        if response.status().is_success() {
            response
                .json::<ScannedMedia>()
                .await
                .map_err(ArachnidShieldError::ReqwestFailed)
        } else {
            Err(ArachnidShieldError::APIError(
                response
                    .json::<ErrorDetail>()
                    .await
                    .expect("error should be parsed as an ErrorDetail"),
            ))
        }
    }

    pub async fn download_and_scan_media_from_url(
        &self,
        url: &str,
    ) -> Result<ScannedMedia, ArachnidShieldError> {
        self.download_and_scan_media_from_url_with_config(ScanMediaFromUrl::new(url))
            .await
    }

    /// Given a `ScanMediaFromUrl` request, send it to the Arachnid Shield API.
    pub async fn download_and_scan_media_from_url_with_config(
        &self,
        params: ScanMediaFromUrl,
    ) -> Result<ScannedMedia, ArachnidShieldError> {
        let response = self
            .client
            .post(self.endpoint("url")?)
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .body(params.body())
            .send()
            .await?;

        if response.status().is_success() {
            response
                .json::<ScannedMedia>()
                .await
                .map_err(ArachnidShieldError::ReqwestFailed)
        } else {
            Err(ArachnidShieldError::APIError(
                response
                    .json::<ErrorDetail>()
                    .await
                    .expect("error should be parsed as an ErrorDetail"),
            ))
        }
    }

    /// Given the PDQ of some media, scan the contents for CSAM using the Arachnid Shield API.
    pub async fn scan_pdq_hashes(
        &self,
        data: &[[u8; 32]],
    ) -> Result<ScannedPdqHashes, ArachnidShieldError> {
        self.scan_pdq_hashes_with_config(ScanPdqHashes::new(data))
            .await
    }

    /// Given a 'ScanPdqHashes' request, send it to the Arachnid Shield API.
    pub async fn scan_pdq_hashes_with_config(
        &self,
        params: ScanPdqHashes,
    ) -> Result<ScannedPdqHashes, ArachnidShieldError> {
        let response = self
            .client
            .post(self.endpoint("pdq")?)
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .body(params.body())
            .send()
            .await?;

        if response.status().is_success() {
            response
                .json::<ScannedPdqHashes>()
                .await
                .map_err(ArachnidShieldError::ReqwestFailed)
        } else {
            Err(ArachnidShieldError::APIError(
                response
                    .json::<ErrorDetail>()
                    .await
                    .expect("error should be parsed as an ErrorDetail"),
            ))
        }
    }
}
