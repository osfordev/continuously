#[derive(Debug, Clone)]
pub struct GoogleCloudBlob {
    bucket_name: String,
    object_name: String,
    // {BUCKET_NAME}/o?uploadType=resumable&name=${OBJECT_NAME}"
    // BUCKET_NAME="logical-volume-backups"
    // OBJECT_NAME="${HOSTNAME}/${VG_NAME}/${LV_NAME}/${LV_TIMESTAMP}"
    // access_token_bundle: AccessTokenBundle,
    // blob_size: u64,
    // content_mime_type: String,
}
impl GoogleCloudBlob {
    pub fn new(
        // access_token_bundle: AccessTokenBundle,
        bucket_name: String,
        object_name: String,
        // blob_size: u64,
        // content_mime_type: String,
    ) -> Self {
        GoogleCloudBlob {
            bucket_name,
            object_name,
            // access_token_bundle,
            // blob_size,
            // content_mime_type,
        }
    }
    // pub fn get_access_token_bundle(&self) -> &AccessTokenBundle {
    //     &self.access_token_bundle
    // }
    // pub fn get_content_mime_type(&self) -> &str {
    //     &self.content_mime_type
    // }
    pub fn get_bucket_name(&self) -> &str {
        &self.bucket_name
    }
    pub fn get_object_name(&self) -> &str {
        &self.object_name
    }
}
