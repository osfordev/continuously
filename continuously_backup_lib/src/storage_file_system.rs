use crate::{
    blob::BlobSize,
    blob_file_system::FileSystemBlob,
    storage::{BlobRead, BlobWrite},
};
use blake2::{Blake2b512, Digest};
use std::io::{Seek};

#[derive(Debug)]
pub struct FileSystemBlobReader {
    buf_reader: std::io::BufReader<std::fs::File>,
    file_size: BlobSize,
    blob: FileSystemBlob,
    current_position: BlobSize,
    current_blake512_hash: [u8; 64],
    pub hasher: Blake2b512,
}
impl FileSystemBlobReader {
    pub fn new(blob: FileSystemBlob) -> std::io::Result<Self> {
        let file: std::fs::File = std::fs::File::open(blob.get_file_path()).unwrap();

        let file_size: BlobSize =
        // { file.metadata().unwrap().len().into() }
        {
            // The OP asked how to get the size of a block device.
            // To get the size of a block device (or any file), you can File.Seek
            // to the end of the file using io.SeekEnd and read the position returned.
            // - https://stackoverflow.com/a/57822606/2011679
            // - https://unix.stackexchange.com/questions/52215/determine-the-size-of-a-block-device/52226#52226
            // - https://stackoverflow.com/questions/2773604/query-size-of-block-device-file-in-python/2774125#2774125
            let mut tmp_file: std::fs::File = std::fs::File::open(blob.get_file_path()).unwrap();
            tmp_file.seek(std::io::SeekFrom::End(0)).unwrap().into()
        }
        ;

        let buf_reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(file);
        let current_position: BlobSize = BlobSize::from(0u64);

        // create a Blake2b512 object
        let hasher = Blake2b512::new();

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

        Ok(FileSystemBlobReader {
            buf_reader,
            file_size,
            blob,
            current_position,
            current_blake512_hash: [0u8; 64],
            hasher,
        })
    }

    pub fn get_file_size(&self) -> BlobSize {
        self.file_size
    }
}

impl From<&FileSystemBlobReader> for crate::state::Source {
    fn from(item: &FileSystemBlobReader) -> Self {
        let state = crate::state::Source::FileSystem {
            size_bytes: item.file_size.into(),
            path: item.blob.get_file_path().to_owned(),
        };

        state
    }
}
// impl crate::Stateable<crate::state::Source> for FileSystemBlobReader {
//     fn to_state(&self) -> crate::state::Source {
//         let state = crate::state::Source::FileSystem {
//             size_bytes: self.file_size,
//             path: self.blob.get_file_path().to_owned(),
//         };

//         state
//     }
// }

#[derive(Debug)]
pub struct FileSystemBlobWriter {
    buf_writer: std::io::BufWriter<std::fs::File>,
    current_position: BlobSize,
    blob: FileSystemBlob,
    mime_type: String,
}
impl FileSystemBlobWriter {
    pub fn new(
        blob: FileSystemBlob,  
        content_mime_type: String,         
        uploaded_bytes: BlobSize,
    ) -> std::io::Result<Self> {
        let mut file = match std::fs::File::create(blob.get_file_path()) {
            std::io::Result::Ok(f) => f,
            std::io::Result::Err(e) => {
                return Err(e);
            }
        };

        let current_position: BlobSize = {
            let file_size = match file.metadata() {
                std::io::Result::Ok(metadata) => metadata.len(),
                std::io::Result::Err(e) => {
                    return Err(e);
                }
            };
            file_size.into()
        };

        match file.seek(std::io::SeekFrom::Start(current_position.into())) {
            std::io::Result::Ok(_) => {}
            std::io::Result::Err(e) => {
                return Err(e);
            }
        }

        let buf_writer: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(file);

        Ok(FileSystemBlobWriter {
            blob,
            buf_writer,
            current_position,
            mime_type: content_mime_type,
        })
    }
}

impl From<&FileSystemBlobWriter> for crate::state::Destination {
    fn from(item: &FileSystemBlobWriter) -> Self {
        let state = crate::state::Destination::FileSystem { 
            size_bytes: item.current_position.clone().into(),
            uploaded_bytes: item.current_position.clone().into(),
            mime: item.mime_type.clone(),
            path: item.blob.get_file_path().to_owned(),
        };

        state
    }
}

impl BlobRead for FileSystemBlobReader {
    fn len(&self) -> BlobSize {
        self.file_size
    }

    fn pos(&self) -> BlobSize {
        self.current_position
    }

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        std::io::Read::read(self, buf)
    }

    fn seek(&mut self, pos: BlobSize) -> std::io::Result<()> {
        match self.buf_reader.seek(std::io::SeekFrom::Start(pos.into())) {
            std::io::Result::Ok(current_position) => {
                self.current_position = current_position.into();
                Ok(())
            }
            std::io::Result::Err(e) => Err(e),
        }
    }
}
impl std::io::Read for FileSystemBlobReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.buf_reader.read(buf) {
            Err(err) => Err(err),
            Ok(read_size) => {
                let friendly_read_size: BlobSize = match read_size.try_into() {
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
}

impl BlobWrite for FileSystemBlobWriter {
    fn pos(&self) -> BlobSize {
        self.current_position
    }

    fn seek(&mut self, pos: BlobSize) -> std::io::Result<()> {
        match self.buf_writer.seek(std::io::SeekFrom::Start(pos.into())) {
            std::io::Result::Ok(current_position) => {
                self.current_position = current_position.into();
                Ok(())
            }
            std::io::Result::Err(e) => Err(e),
        }
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        std::io::Write::write(self, buf)
    }
}
impl std::io::Write for FileSystemBlobWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.buf_writer.write(buf) {
            Err(err) => Err(err),
            Ok(write_size) => {
                let friendly_write_size: BlobSize = match write_size.try_into() {
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

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
