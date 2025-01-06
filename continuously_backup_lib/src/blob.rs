pub enum Blob {
    FileSystem(crate::blob_file_system::FileSystemBlob),
    GoogleCloud(crate::blob_google_cloud::GoogleCloudBlob),
}

impl Blob {
}
