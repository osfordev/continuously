use crate::{
    blob_file_system::FileSystemBlob,
    storage::{BlobRead, BlobWrite},
};
use blake2::{Blake2b512, Digest};
use std::{
    io::{Read, Seek, Write},
};

#[derive(Debug)]
pub struct FileSystemBlobBufReader {
    buf_reader: std::io::BufReader<std::fs::File>,
    file_size: u64,
    blob: FileSystemBlob,
    current_position: u64,
    current_blake512_hash: [u8; 64],
    pub hasher: Blake2b512,
}
impl FileSystemBlobBufReader {
    pub fn new(blob: FileSystemBlob) -> std::io::Result<Self> {
        let file: std::fs::File = std::fs::File::open(blob.get_file_path()).unwrap();
        let file_size = file.metadata().unwrap().len();
        let buf_reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(file);
        let current_position: u64 = 0;

        // create a Blake2b512 object
        let mut hasher = Blake2b512::new();

        // // write input message
        // hasher.update(b"hello world");

        // // read hash digest and consume hasher
        // let res = hasher.clone().finalize();

        // let mut hasher3 = Blake2b512::new_with_prefix(res);

        // // write input message
        // hasher.update(b"hello world");
        // hasher3.update(b"hello world");
        // let res2 = hasher.clone().finalize();
        // let res3 = hasher3.finalize();

        // let current_blake512_hash1: [u8; 64] = res.try_into().unwrap();
        // let current_blake512_hash2: [u8; 64] = res2.try_into().unwrap();
        // let current_blake512_hash3: [u8; 64] = res3.try_into().unwrap();

        Ok(FileSystemBlobBufReader {
            buf_reader,
            file_size,
            blob,
            current_position,
            current_blake512_hash: [0u8; 64],
            hasher,
        })
    }

    pub fn get_file_size(&self) -> u64 {
        self.file_size
    }
}

impl crate::Stateable<crate::state::Source> for FileSystemBlobBufReader {
    fn to_state(&self) -> crate::state::Source {
        let state = crate::state::Source::FileSystem {
            size_bytes: self.file_size,
            path: self.blob.get_file_path().to_owned(),
        };

        state
    }
}

#[derive(Debug)]
pub struct FileSystemBlobBufWriter {
    buf_writer: std::io::BufWriter<std::fs::File>,
    current_position: u64,
    blob: FileSystemBlob,
}
impl FileSystemBlobBufWriter {
    pub fn new(blob: FileSystemBlob) -> std::io::Result<Self> {
        let mut file = match std::fs::File::create(blob.get_file_path()) {
            std::io::Result::Ok(f) => f,
            std::io::Result::Err(e) => {
                return Err(e);
            }
        };

        let current_position = {
            let file_size = match file.metadata() {
                std::io::Result::Ok(metadata) => metadata.len(),
                std::io::Result::Err(e) => {
                    return Err(e);
                }
            };
            file_size
        };

        match file.seek(std::io::SeekFrom::Start(current_position)) {
            std::io::Result::Ok(_) => {}
            std::io::Result::Err(e) => {
                return Err(e);
            }
        }

        let buf_writer: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(file);

        Ok(FileSystemBlobBufWriter {
            blob,
            buf_writer,
            current_position,
        })
    }
}

impl crate::Stateable<crate::state::Destination> for FileSystemBlobBufWriter {
    fn to_state(&self) -> crate::state::Destination {
        todo!()
    }
}

impl BlobRead for FileSystemBlobBufReader {
    fn len(&self) -> u64 {
        self.file_size
    }

    fn pos(&self) -> u64 {
        self.current_position
    }

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.buf_reader.read(buf) {
            Err(err) => Err(err),
            Ok(read_size) => {
                let friendly_read_size: u64 = match read_size.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Unable to convert usize to u64",
                        ));
                    }
                };

                self.current_position += friendly_read_size;

                Ok(read_size)
            }
        }
    }

    fn seek(&mut self, pos: u64) -> std::io::Result<()> {
        match self.buf_reader.seek(std::io::SeekFrom::Start(pos)) {
            std::io::Result::Ok(current_position) => {
                self.current_position = current_position;
                Ok(())
            }
            std::io::Result::Err(e) => Err(e),
        }
    }
}

impl BlobWrite for FileSystemBlobBufWriter {
    fn pos(&self) -> u64 {
        self.current_position
    }

    fn seek(&mut self, pos: u64) -> std::io::Result<()> {
        match self.buf_writer.seek(std::io::SeekFrom::Start(pos)) {
            std::io::Result::Ok(current_position) => {
                self.current_position = current_position;
                Ok(())
            }
            std::io::Result::Err(e) => Err(e),
        }
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.buf_writer.write(buf) {
            Err(err) => Err(err),
            Ok(write_size) => {
                let friendly_write_size: u64 = match write_size.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Unable to convert usize to u64",
                        ));
                    }
                };

                self.current_position += friendly_write_size;

                Ok(write_size)
            }
        }
    }
}
