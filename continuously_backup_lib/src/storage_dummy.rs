use log::{debug, info, trace, warn};

use crate::{
    blob::{BlobSize, DummyBlob},
    storage::{BlobRead, BlobWrite},
};

#[derive(Debug)]
pub struct DummyBlobReader {
    blob: DummyBlob,
    blob_size: BlobSize,
    current_position: BlobSize,
}
impl DummyBlobReader {
    pub fn new(blob: DummyBlob, blob_size: BlobSize) -> std::io::Result<Self> {
        Ok(DummyBlobReader {
            blob,
            blob_size,
            current_position: BlobSize::from(0u64),
        })
    }
}

impl From<&DummyBlobReader> for crate::state::Source {
    fn from(item: &DummyBlobReader) -> Self {
        let state = crate::state::Source::Dummy {
            size_bytes: (&item.blob_size).into(),
        };

        state
    }
}

#[derive(Debug)]
pub struct DummyBlobWriter {
    blob: DummyBlob,
    current_position: BlobSize,
}
impl DummyBlobWriter {
    pub fn new(blob: DummyBlob) -> std::io::Result<Self> {
        Ok(DummyBlobWriter {
            blob,
            current_position: BlobSize::from(0u64),
        })
    }
}

impl From<&DummyBlobWriter> for crate::state::Destination {
    fn from(item: &DummyBlobWriter) -> Self {
        let state = crate::state::Destination::Dummy {};

        state
    }
}

impl BlobRead for DummyBlobReader {
    fn len(&self) -> BlobSize {
        self.current_position
    }

    fn pos(&self) -> BlobSize {
        self.current_position
    }

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        std::io::Read::read(self, buf)
    }

    fn seek(&mut self, pos: BlobSize) -> std::io::Result<()> {
        self.current_position = pos;
        Ok(())
    }
}

impl std::io::Read for DummyBlobReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let buf_len: usize = buf.len();
        let friendly_buf_len: BlobSize = match buf_len.try_into() {
            Ok(v) => v,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unable to convert usize to u64",
                ));
            }
        };

        for buf_index in 1..buf_len {
            buf[buf_index] = 0;
        }

        self.current_position += friendly_buf_len;

        Ok(buf_len)
    }
}

impl BlobWrite for DummyBlobWriter {
    fn pos(&self) -> BlobSize {
        self.current_position
    }

    fn seek(&mut self, pos: BlobSize) -> std::io::Result<()> {
        self.current_position = pos;
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        std::io::Write::write(self, buf)
    }
}

impl std::io::Write for DummyBlobWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf_len: usize = buf.len();
        let friendly_buf_len: BlobSize = match buf_len.try_into() {
            Ok(v) => v,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unable to convert usize to u64",
                ));
            }
        };

        self.current_position += friendly_buf_len;

        debug!(
            "DummyBlobWriter: write chunk of {} bytes, current_position is {}.",
            friendly_buf_len, self.current_position
        );

        Ok(buf_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
