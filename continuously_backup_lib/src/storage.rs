use crate::blob::BlobSize;

pub trait BlobRead {
    fn len(&self) -> BlobSize;
    fn pos(&self) -> BlobSize;
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
    fn seek(&mut self, pos: BlobSize) -> std::io::Result<()>;
}

pub trait BlobWrite {
    fn pos(&self) -> BlobSize;
    fn seek(&mut self, pos: BlobSize) -> std::io::Result<()>;
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
}

#[derive(Debug)]
pub enum BlobReader {
    Dummy(crate::storage_dummy::DummyBlobReader),
    FileSystem(crate::storage_file_system::FileSystemBlobReader),
    GoogleCloud(crate::storage_google_cloud::GoogleCloudBlobReader),
}
impl BlobRead for BlobReader {
    fn len(&self) -> BlobSize {
        match self {
            BlobReader::Dummy(reader) => reader.len(),
            BlobReader::FileSystem(reader) => reader.len(),
            BlobReader::GoogleCloud(reader) => reader.len(),
        }
    }

    fn pos(&self) -> BlobSize {
        match self {
            BlobReader::Dummy(reader) => reader.pos(),
            BlobReader::FileSystem(reader) => reader.pos(),
            BlobReader::GoogleCloud(reader) => reader.pos(),
        }
    }

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            BlobReader::Dummy(reader) => reader.read(buf),
            BlobReader::FileSystem(reader) => reader.read(buf),
            BlobReader::GoogleCloud(reader) => reader.read(buf),
        }
    }

    fn seek(&mut self, pos: BlobSize) -> std::io::Result<()> {
        match self {
            BlobReader::Dummy(reader) => reader.seek(pos),
            BlobReader::FileSystem(reader) => reader.seek(pos),
            BlobReader::GoogleCloud(reader) => reader.seek(pos),
        }
    }
}
impl std::io::Read for BlobReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            BlobReader::Dummy(reader) => std::io::Read::read(reader, buf),
            BlobReader::FileSystem(reader) => std::io::Read::read(reader, buf),
            BlobReader::GoogleCloud(reader) => std::io::Read::read(reader, buf),
        }
    }
}

#[derive(Debug)]
pub enum BlobWriter {
    Dummy(crate::storage_dummy::DummyBlobWriter),
    FileSystem(crate::storage_file_system::FileSystemBlobWriter),
    GoogleCloud(crate::storage_google_cloud::GoogleCloudBlobWriter),
}
impl BlobWrite for BlobWriter {
    fn pos(&self) -> BlobSize {
        match self {
            BlobWriter::Dummy(writer) => writer.pos(),
            BlobWriter::FileSystem(writer) => writer.pos(),
            BlobWriter::GoogleCloud(writer) => writer.pos(),
        }
    }

    fn seek(&mut self, pos: BlobSize) -> std::io::Result<()> {
        match self {
            BlobWriter::Dummy(writer) => writer.seek(pos),
            BlobWriter::FileSystem(writer) => writer.seek(pos),
            BlobWriter::GoogleCloud(writer) => writer.seek(pos),
        }
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            BlobWriter::Dummy(writer) => writer.write(buf),
            BlobWriter::FileSystem(writer) => writer.write(buf),
            BlobWriter::GoogleCloud(writer) => writer.write(buf),
        }
    }
}
impl std::io::Write for BlobWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            BlobWriter::Dummy(writer) => std::io::Write::write(writer, buf),
            BlobWriter::FileSystem(writer) => std::io::Write::write(writer, buf),
            BlobWriter::GoogleCloud(writer) => std::io::Write::write(writer, buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
