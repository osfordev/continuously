use crate::{blob_google_cloud::GoogleCloudBlob, storage::BlobRead, storage::BlobWrite};

// use std::io::{BufReader, Read, Seek, Write};

fn initiate_resumable_upload_session(
    blob: &GoogleCloudBlob,
    access_token: &str,
    content_mime_type: &str,
) -> url::Url {
    let session_url: url::Url = api_initiate_resumable_upload_session(
        access_token,
        blob.get_bucket_name(),
        blob.get_object_name(),
        content_mime_type,
    )
    .unwrap();

    session_url
}

#[derive(Debug)]
pub struct GoogleCloudBlobBufReader {
    blob: GoogleCloudBlob,
    blob_size: u64,
    current_position: u64,
}
impl GoogleCloudBlobBufReader {
    pub fn new(
        blob: GoogleCloudBlob,
        // access_token_bundle: AccessTokenBundle,
        // content_mime_type: &str,
    ) -> std::io::Result<Self> {
        // let session_url: url::Url = initiate_resumable_upload_session(
        //     blob,
        //     &access_token_bundle.access_token,
        //     content_mime_type,
        // );

        // let file = std::fs::File::open(file_path).unwrap();
        // let file_size = file.metadata().unwrap().len();
        // let buf_reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(file);
        Ok(GoogleCloudBlobBufReader {
            blob,
            blob_size: 0,
            current_position: 0,
        })
    }
}

impl crate::Stateable<crate::state::Source> for GoogleCloudBlobBufReader {
    fn to_state(&self) -> crate::state::Source {
        todo!()
    }
}

#[derive(Debug)]
pub struct GoogleCloudBlobBufWriter {
    blob: GoogleCloudBlob,
    blob_size: u64,
    current_position: u64,
    session_url: url::Url,
    mime_type: String,
}
impl GoogleCloudBlobBufWriter {
    pub fn new(
        blob: GoogleCloudBlob,
        content_mime_type: String,
        blob_size: u64,
        session_url: url::Url,
       uploaded_bytes: u64,
    ) -> Self {
        GoogleCloudBlobBufWriter {
            session_url,
            current_position: uploaded_bytes,
            blob_size,
            blob,
            mime_type: content_mime_type,
        }
    }

    pub fn new_with_access_token(
        blob: GoogleCloudBlob,
        content_mime_type: String,
        blob_size: u64,
        access_token_bundle: AccessTokenBundle,
    ) -> std::io::Result<Self> {
        let session_url: url::Url = initiate_resumable_upload_session(
            &blob,
            &access_token_bundle.access_token,
            &content_mime_type,
        );

        Ok(GoogleCloudBlobBufWriter {
            session_url,
            current_position: 0,
            blob_size,
            blob,
            mime_type: content_mime_type,
        })
    }
}

impl crate::Stateable<crate::state::Destination> for GoogleCloudBlobBufWriter {
    fn to_state(&self) -> crate::state::Destination {
        let state = crate::state::Destination::GoogleCloud {
            bucket_name: self.blob.get_bucket_name().to_owned(),
            object_name: self.blob.get_object_name().to_owned(),
            size_bytes: self.blob_size,
            mime: self.mime_type.clone(),
            session_url: self.session_url.as_str().to_owned(),
            uploaded_bytes: self.current_position,
        };

        state
    }
}

impl BlobRead for GoogleCloudBlobBufReader {
    fn len(&self) -> u64 {
        self.blob_size
    }

    fn pos(&self) -> u64 {
        self.current_position
    }

    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not implemented yet",
        ))
    }

    fn seek(&mut self, _pos: u64) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not implemented yet",
        ))
    }
}

impl BlobWrite for GoogleCloudBlobBufWriter {
    fn pos(&self) -> u64 {
        self.current_position
    }

    fn seek(&mut self, pos: u64) -> std::io::Result<()> {
        // TODO: check pos range and multiplier
        self.current_position = pos;

        Ok(())
    }

    fn write(&mut self, chunk: &[u8]) -> std::io::Result<usize> {
        // self.buf_writer.write(buf)
        api_upload_chunk(
            &self.session_url,
            chunk,
            self.current_position,
            self.blob_size,
        )
        .unwrap();

        let chunk_size: u64 = chunk.len().try_into().unwrap();
        self.current_position += chunk_size;

        Ok(chunk.len())
    }
}

// absolute-garden-272819-d3d737919552.local.json

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleCloudPlatformServiceAccount {
    //"type": "service_account",
    #[serde(rename = "type")]
    type_name: String,

    //"project_id": "absolute-garden-272819",
    #[serde(rename = "project_id")]
    project_id: String,

    //"private_key_id": "d3d73791955275817c6efa0f426b4e01c11d290f",
    #[serde(rename = "private_key_id")]
    private_key_id: String,

    //"private_key": "-----BEGIN PRIVATE KEY-----\nMII...jg=\n-----END PRIVATE KEY-----\n",
    #[serde(rename = "private_key")]
    private_key: String,

    //"client_email": "lv-upload@absolute-garden-272819.iam.gserviceaccount.com",
    #[serde(rename = "client_email")]
    client_email: String,

    //"client_id": "112557070002931692753",
    #[serde(rename = "client_id")]
    client_id: String,

    //"auth_uri": "https://accounts.google.com/o/oauth2/auth",
    #[serde(rename = "auth_uri")]
    auth_uri: String,

    //"token_uri": "https://oauth2.googleapis.com/token",
    #[serde(rename = "token_uri")]
    token_uri: String,

    //"auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
    #[serde(rename = "auth_provider_x509_cert_url")]
    auth_provider_x509_cert_url: String,

    //"client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/lv-upload%40absolute-garden-272819.iam.gserviceaccount.com",
    #[serde(rename = "client_x509_cert_url")]
    client_x509_cert_url: String,

    //"universe_domain": "googleapis.com"
    #[serde(rename = "universe_domain")]
    universe_domain: String,
}
impl GoogleCloudPlatformServiceAccount {
    pub fn from_file(
        service_account_json_file: &str,
    ) -> Result<Self, GoogleCloudPlatformServiceAccountFromFileError> {
        let open_result = std::fs::File::open(service_account_json_file)
            .map_err(GoogleCloudPlatformServiceAccountFromFileError::ReadError);
        match open_result {
            Err(err) => Err(err),
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                let parse_result = ureq::serde_json::from_reader(reader)
                    .map_err(GoogleCloudPlatformServiceAccountFromFileError::ParseError);
                match parse_result {
                    Ok(gcp_service_account) => Ok(gcp_service_account),
                    Err(err) => Err(err),
                }
            }
        }
    }
}
#[derive(Debug)]
pub enum GoogleCloudPlatformServiceAccountFromFileError {
    ReadError(std::io::Error),
    ParseError(ureq::serde_json::Error),
}

#[non_exhaustive]
pub struct Scope;
impl Scope {
    /// read-write - allows access to read and change data, but not metadata like IAM policies.
    ///
    /// See [Cloud Storage OAuth 2.0 scopes](https://cloud.google.com/storage/docs/oauth-scopes)
    pub const DEVSTORAGE_READ_WRITE: &str =
        &"https://www.googleapis.com/auth/devstorage.read_write";
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    #[serde(rename = "iat")]
    issued_at_time_unix: u64,

    #[serde(rename = "exp")]
    expiration_time_unix: u64,

    #[serde(rename = "scope")]
    scope: String,

    #[serde(rename = "aud")]
    audience: String,

    #[serde(rename = "iss")]
    issuer: String,
}

pub fn generate_jwt_refresh_token(
    gcp_service_account: &GoogleCloudPlatformServiceAccount,
) -> String {
    //
    //# JWT Header
    // ```json
    // {
    //   "typ": "JWT",
    //   "alg": "RS256",
    //   "kid": "d3d73791955275817c6efa0f426b4e01c11d290f"
    // }
    // ```
    //
    //# JWT Payload
    // ```json
    // {
    //   "iat": 1735834673,
    //   "exp": 1735838273,
    //   "scope": "https://www.googleapis.com/auth/devstorage.read_write",
    //   "aud": "https://oauth2.googleapis.com/token",
    //   "iss": "lv-upload@absolute-garden-272819.iam.gserviceaccount.com"
    // }
    // ```
    //

    let mut jwt_header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    jwt_header.typ = Option::Some("JWT".to_owned());
    jwt_header.kid = Option::Some(gcp_service_account.private_key_id.to_owned());

    let private_key =
        jsonwebtoken::EncodingKey::from_rsa_pem(gcp_service_account.private_key.as_bytes())
            .unwrap();

    let now: std::time::SystemTime = std::time::SystemTime::now();
    let iat: u64 = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let exp: u64 = iat + 60 * 60;

    // Claims is a struct that implements Serialize
    let jwt_payload_claims: Claims = Claims {
        issued_at_time_unix: iat,
        expiration_time_unix: exp,
        scope: String::from(Scope::DEVSTORAGE_READ_WRITE),
        audience: gcp_service_account.token_uri.to_owned(),
        issuer: gcp_service_account.client_email.to_owned(),
    };

    let jwt_token = jsonwebtoken::encode(&jwt_header, &jwt_payload_claims, &private_key).unwrap();

    jwt_token
}

pub fn api_get_access_token(refresh_token: &str) -> std::io::Result<AccessTokenBundle> {
    let resp = ureq::post("https://oauth2.googleapis.com/token")
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", refresh_token),
        ])
        .unwrap();
    let access_token_bundle: AccessTokenBundle = resp.into_json().unwrap();
    Ok(access_token_bundle)
}

//
// https://cloud.google.com/storage/docs/performing-resumable-uploads#initiate-session
//
pub fn api_initiate_resumable_upload_session(
    access_token: &str,
    bucket_name: &str,
    object_name: &str,
    content_mime_type: &str,
    // blob_size: u64,
) -> std::io::Result<url::Url> {
    // curl \
    // --silent \
    // --fail \
    // --include \
    // --request POST \
    // --data '{"contentType": "application/octet-stream"}' \
    // --header "Authorization: Bearer ${ACCESS_TOKEN}" \
    // --header "Content-Type: application/json" \
    // "https://storage.googleapis.com/upload/storage/v1/b/${BUCKET_NAME}        /o?uploadType=resumable&name=${OBJECT_NAME}" \
    //  https://storage.googleapis.com/upload/storage/v1/b/logical-volume-backups/o?uploadType=resumable&name=test/README.md
    // | tee upload-response.txt

    let url = url::Url::parse("https://storage.googleapis.com/upload/storage/v1/b/").unwrap();
    let bucket_name_part = format!("{}/", bucket_name);
    let url: url::Url = url.join(bucket_name_part.as_str()).unwrap();
    let mut url = url.join("o").unwrap();
    url.query_pairs_mut()
        .append_pair("uploadType", "resumable")
        .append_pair("name", object_name);

    let url: &str = url.as_str();

    let authorization = format!("Bearer {}", access_token);

    // TODO: Optional headers that you can add to the request include X-Upload-Content-Type and X-Upload-Content-Length.
    let resp = ureq::post(url)
        .set("Authorization", &authorization)
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            // https://cloud.google.com/storage/docs/json_api/v1/objects#resource-representations
            "contentType": content_mime_type,
        }))
        .unwrap();
    match resp.header("Location") {
        None => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Integrity violation. API response does not contain a Location header",
        )),
        Some(data) => Ok(url::Url::parse(data).unwrap()),
    }
}

//
// https://cloud.google.com/storage/docs/performing-resumable-uploads#cancel-upload
//
pub fn api_cancel_upload(session_url: &url::Url) -> std::io::Result<()> {
    // curl -i -X DELETE -H "Content-Length: 0" \
    //   "SESSION_URI"

    let session_url: &str = session_url.as_str();

    let resp_result = ureq::delete(session_url).call();
    match resp_result {
        Ok(response) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "Unable to close upload session. Got unexpected status code: {} {}",
                response.status(),
                response.status_text()
            ),
        )),
        Err(ureq::Error::Status(status, response)) => {
            if status == 499 {
                // If successful, the response contains a 499 status code.
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!(
                        "Unable to close upload session. Got unexpected status code: {} {}",
                        status,
                        response.status_text()
                    ),
                ))
            }
        }
        Err(err) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "Unable to close upload session. Underlying error: {}",
                err.to_string()
            ),
        )),
    }
}

pub enum ResumableUploadStatus {
    Completed,
    Pending(u64),
}

//
// https://cloud.google.com/storage/docs/performing-resumable-uploads#status-check
//
pub fn api_check_status_of_resumable_upload(
    session_url: &url::Url,
    // blob_size: u64,
) -> std::io::Result<ResumableUploadStatus> {
    // curl -i -X PUT \
    // -H "Content-Length: 0" \
    // -H "Content-Range: bytes */OBJECT_SIZE" \
    // "SESSION_URI"

    let session_url: &str = session_url.as_str();

    let resp_result = ureq::put(session_url)
        .set("Content-Length", "0")
        // .set("Content-Range", &format!("bytes */{}", blob_size))
        // If you don't know the full size of your object, use * for this value.
        .set("Content-Range", "bytes */*")
        .call();
    match resp_result {
        Ok(response) => {
            let status = response.status();
            let status_text = response.status_text();

            print!("Got status code: {} {}", status, status_text);

            if status == 200 || status == 201 {
                Ok(ResumableUploadStatus::Completed)
            } else if status == 308 {
                match response.header("Range") {
                    None => Ok(ResumableUploadStatus::Pending(0)),
                    Some(range) => {
                        print!("Range: {}", range);
                        Ok(ResumableUploadStatus::Pending(0))
                    }
                }
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!(
                    "Unable to check status of upload session. Got unexpected status code: {} {}",
                    response.status(),
                    response.status_text()
                ),
                ))
            }
        }
        Err(ureq::Error::Status(status, response)) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "Unable to check status of upload session. Got unexpected status code: {} {}",
                status,
                response.status_text()
            ),
        )),
        Err(err) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "Unable to close upload session. Underlying error: {}",
                err.to_string()
            ),
        )),
    }
}

///
/// # Examples
///
/// ```json
/// {
///     "access_token":"ya29...mu7",
///     "expires_in":3599,
///     "token_type":"Bearer"
/// }
/// ```
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AccessTokenBundle {
    #[serde(rename = "access_token")]
    pub access_token: String,

    #[serde(rename = "expires_in")]
    pub expires_in: u32,

    #[serde(rename = "token_type")]
    pub token_type: String,
}

#[derive(Debug)]
pub enum UploadChunkError {
    ApiContractViolation(String),
    ApiInteraction(ureq::Error),
    Argument(String),
}

const CHUNK_SIZE_MULTIPLIER: u64 = 256 * 1024;

pub fn api_upload_chunk(
    session_url: &url::Url,
    chunk: &[u8],
    blob_offset: u64,
    blob_size: u64,
) -> Result<ResumableUploadStatus, UploadChunkError> {
    let session_url: &str = session_url.as_str();

    let chunk_size: u64 = chunk.len().try_into().unwrap();
    if chunk_size < 1 {
        return Err(UploadChunkError::Argument(String::from(
            "The chunk size must be greater than zero.",
        )));
    }

    let is_last_chunk: bool = chunk_size % CHUNK_SIZE_MULTIPLIER != 0;

    let next_byte: u64 = blob_offset;
    let last_byte = next_byte + chunk_size;

    if is_last_chunk && last_byte != blob_size {
        return Err(
            UploadChunkError::Argument(
                String::from(
                    "The chunk size should be a multiple of 256 KiB (256 x 1024 bytes), unless it's the last chunk that completes the upload. See https://cloud.google.com/storage/docs/performing-resumable-uploads#chunked-upload for details."
                )
            )
        );
    }

    let content_range = format!("bytes {}-{}/{}", next_byte, last_byte - 1, blob_size);

    match ureq::put(session_url)
        .set("Content-Length", &chunk_size.to_string())
        .set("Content-Range", &content_range)
        .send_bytes(chunk)
    {
        Ok(response) => {
            //
            let response_status = response.status();
            match response_status {
                308 => Ok(ResumableUploadStatus::Pending(last_byte)),
                200 | 201 => Ok(ResumableUploadStatus::Completed),
                _ => Err(UploadChunkError::ApiContractViolation(format!(
                    "Unexpected response status: {}",
                    response_status
                ))),
            }
        }
        Err(err) => Err(UploadChunkError::ApiInteraction(err)),
    }
}
