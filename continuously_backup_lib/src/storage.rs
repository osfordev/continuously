pub trait BlobRead {
    fn len(&self) -> u64;
    fn pos(&self) -> u64;
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
    fn seek(&mut self, pos: u64) -> std::io::Result<()>;
}

pub trait BlobWrite {
    fn pos(&self) -> u64;
    fn seek(&mut self, pos: u64) -> std::io::Result<()>;
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
}

#[derive(Debug)]
pub enum BlobReader {
    FileSystem(crate::storage_file_system::FileSystemBlobBufReader),
    GoogleCloud(crate::storage_google_cloud::GoogleCloudBlobBufReader),
}
impl BlobRead for BlobReader {
    fn len(&self) -> u64 {
        match self {
            BlobReader::FileSystem(reader) => reader.len(),
            BlobReader::GoogleCloud(reader) => reader.len(),
        }
    }

    fn pos(&self) -> u64 {
        match self {
            BlobReader::FileSystem(reader) => reader.pos(),
            BlobReader::GoogleCloud(reader) => reader.pos(),
        }
    }

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            BlobReader::FileSystem(reader) => reader.read(buf),
            BlobReader::GoogleCloud(reader) => reader.read(buf),
        }
    }

    fn seek(&mut self, pos: u64) -> std::io::Result<()> {
        match self {
            BlobReader::FileSystem(reader) => reader.seek(pos),
            BlobReader::GoogleCloud(reader) => reader.seek(pos),
        }
    }
}

#[derive(Debug)]
pub enum BlobWriter {
    FileSystem(crate::storage_file_system::FileSystemBlobBufWriter),
    GoogleCloud(crate::storage_google_cloud::GoogleCloudBlobBufWriter),
}
impl BlobWrite for BlobWriter {
    fn pos(&self) -> u64 {
        match self {
            BlobWriter::FileSystem(writer) => writer.pos(),
            BlobWriter::GoogleCloud(writer) => writer.pos(),
        }
    }

    fn seek(&mut self, pos: u64) -> std::io::Result<()> {
        match self {
            BlobWriter::FileSystem(writer) => writer.seek(pos),
            BlobWriter::GoogleCloud(writer) => writer.seek(pos),
        }
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            BlobWriter::FileSystem(writer) => writer.write(buf),
            BlobWriter::GoogleCloud(writer) => writer.write(buf),
        }
    }
}
