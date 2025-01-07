#[derive(Debug, Clone)]
pub struct DummyBlob;
impl DummyBlob {}

#[derive(Debug, Clone)]
pub struct FileSystemBlob {
    file_path: String,
}
impl FileSystemBlob {
    pub fn new(file_path: String) -> Self {
        FileSystemBlob { file_path }
    }
    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }
}

#[derive(Debug, Clone)]
pub struct GoogleCloudBlob {
    bucket_name: String,
    object_name: String,
}
impl GoogleCloudBlob {
    pub fn new(
        bucket_name: String,
        object_name: String,
    ) -> Self {
        GoogleCloudBlob {
            bucket_name,
            object_name,
        }
    }
    pub fn get_bucket_name(&self) -> &str {
        &self.bucket_name
    }
    pub fn get_object_name(&self) -> &str {
        &self.object_name
    }
}

pub enum Blob {
    Dummy(DummyBlob),
    FileSystem(FileSystemBlob),
    GoogleCloud(crate::blob_google_cloud::GoogleCloudBlob),
}

impl Blob {}

#[derive(Debug, Clone, Copy)]
pub struct BlobSize(u64);

impl std::fmt::Display for BlobSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::ops::Add for BlobSize {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut res: u64 = (&self).into();
        let other: u64 = (&other).into();
        res += other;
        BlobSize(res)
    }
}

impl std::ops::AddAssign for BlobSize {
    fn add_assign(&mut self, rhs: Self) {
        let rhs: u64 = (&rhs).into();
        self.0 += rhs;
    }
}

impl From<u64> for BlobSize {
    fn from(item: u64) -> Self {
        BlobSize(item)
    }
}

impl From<usize> for BlobSize {
    fn from(item: usize) -> Self {
        let friendly_item: u64 = match item.try_into() {
            Ok(v) => v,
            Err(_) => panic!("Unable to convert BlobSize to usize"),
        };
        BlobSize(friendly_item)
    }
}

impl From<BlobSize> for u64 {
    fn from(item: BlobSize) -> Self {
        item.0
    }
}

impl From<&BlobSize> for u64 {
    fn from(item: &BlobSize) -> Self {
        item.0
    }
}

impl Into<usize> for BlobSize {
    fn into(self) -> usize {
        match self.0.try_into() {
            Ok(v) => v,
            Err(_) => panic!("Unable to convert usize to BlobSize"),
        }
    }
}
