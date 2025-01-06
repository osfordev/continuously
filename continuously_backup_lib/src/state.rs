#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind")]
pub enum Source {
    #[serde(rename = "file")]
    FileSystem {
        #[serde(rename = "fileTotalBytes")]
        size_bytes: u64,
        #[serde(rename = "filePath")]
        path: String,
    },
    #[serde(rename = "googleCloud")]
    GoogleCloud {
        #[serde(rename = "googleCloudBucketName")]
        bucket_name: String,
        #[serde(rename = "googleCloudObjectName")]
        object_name: String,
        #[serde(rename = "googleCloudTotalBytes")]
        size_bytes: u64,
        #[serde(rename = "googleCloudMimeType")]
        mime: String,
        #[serde(rename = "googleCloudSessionUrl")]
        session_url: String,
        #[serde(rename = "googleCloudUploadedBytes")]
        uploaded_bytes: u64,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind")]
pub enum Destination {
    #[serde(rename = "file")]
    FileSystem {
        #[serde(rename = "fileTotalBytes")]
        size_bytes: u64,
        #[serde(rename = "filePath")]
        path: String,
    },
    #[serde(rename = "googleCloud")]
    GoogleCloud {
        #[serde(rename = "googleCloudBucketName")]
        bucket_name: String,
        #[serde(rename = "googleCloudObjectName")]
        object_name: String,
        #[serde(rename = "googleCloudTotalBytes")]
        size_bytes: u64,
        #[serde(rename = "googleCloudMimeType")]
        mime: String,
        #[serde(rename = "googleCloudSessionUrl")]
        session_url: String,
        #[serde(rename = "googleCloudUploadedBytes")]
        uploaded_bytes: u64,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Extra<T> {
    #[serde(flatten)]
    pub extra: T,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionState {
    #[serde(rename = "source")]
    pub source: Extra<Source>,

    #[serde(rename = "destination")]
    pub destination: Extra<Destination>,
}

impl SessionState {
    // pub fn new(source: &dyn BlobReader, destination: &dyn BlobWriter) -> Self {
    //     let s: Source;
    //     let d: Destination;

    //     if TypeId::of::<storage_google_cloud::GoogleCloudBlobBufReader>() == source.type_id() {
    //         //
    //     }

    //     let session = Session {
    //         content_mime_type: String::from("ddd"),
    //         source: Extra { extra: s },
    //         destination: Extra { extra: d },
    //     };

    //     session
    // }

    pub fn println(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        println!("{}", json);
    }
}
