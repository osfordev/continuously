use crate::{
    blob::{BlobSize, DummyBlob},
    blob_file_system::FileSystemBlob,
    blob_google_cloud::GoogleCloudBlob,
    storage::{BlobRead, BlobReader, BlobWrite, BlobWriter},
    storage_dummy::{DummyBlobReader, DummyBlobWriter},
    storage_file_system::{FileSystemBlobReader, FileSystemBlobWriter},
    storage_google_cloud::{GoogleCloudBlobReader, GoogleCloudBlobWriter},
};

pub struct Session {
    source_reader: BlobReader,
    destination_writer: BlobWriter,
}
impl Session {
    pub fn new(source_reader: BlobReader, destination_writer: BlobWriter) -> Self {
        // let source_reader: Source = {
        //     match &source_blob {
        //         Blob::FileSystem(blob) => Source::FileSystem(
        //             crate::storage_file_system::FileSystemBlobBufReader::new(blob.get_file_path())
        //                 .unwrap(),
        //         ),
        //         Blob::GoogleCloud(blob) => Source::GoogleCloud(
        //             crate::storage_google_cloud::GoogleCloudBlobBufReader::new(blob).unwrap(),
        //         ),
        //     }
        // };
        // let destination_writer: Destination = {
        //     match &destination_blob {
        //         Blob::FileSystem(blob) => Destination::FileSystem(
        //             crate::storage_file_system::FileSystemBlobBufWriter::new(blob.get_file_path())
        //                 .unwrap(),
        //         ),
        //         Blob::GoogleCloud(blob) => Destination::GoogleCloud(
        //             crate::storage_google_cloud::GoogleCloudBlobBufWriter::new(blob).unwrap(),
        //         ),

        //     }
        // };

        //
        let session = Session {
            source_reader,
            destination_writer,
        };

        session
    }

    pub fn from_state(state: &crate::state::SessionState) -> Result<Self, SessionCreateError> {
        let source_reader: Result<BlobReader, SessionCreateError> = {
            match &state.source.extra {
                crate::state::Source::Dummy { size_bytes } => {
                    //
                    Ok(BlobReader::Dummy(
                        DummyBlobReader::new(DummyBlob, (*size_bytes).into()).unwrap(),
                    ))
                }
                crate::state::Source::FileSystem { size_bytes, path } => {
                    let reader: Result<FileSystemBlobReader, SessionCreateError> = {
                        let file_path: String = path.to_owned();
                        let source_blob = FileSystemBlob::new(file_path);
                        Ok(FileSystemBlobReader::new(source_blob).unwrap())
                    };

                    if reader.is_err() {
                        return Err(reader.unwrap_err());
                    }

                    // TODO: compare size_bytes
                    // TODO: compare creation_date
                    // TODO: calc and compare checksum

                    Ok(BlobReader::FileSystem(reader.unwrap()))
                }
                crate::state::Source::GoogleCloud {
                    bucket_name,
                    object_name,
                    size_bytes,
                    mime,
                    session_url,
                    uploaded_bytes: u64,
                } => {
                    // let reader: GoogleCloudBlobBufReader;

                    // BlobReader::GoogleCloud(reader)
                    Err(SessionCreateError::Unknown)
                }
            }
        };
        let destination_writer: Result<BlobWriter, SessionCreateError> = {
            match &state.destination.extra {
                crate::state::Destination::Dummy {} => {
                    Ok(BlobWriter::Dummy(DummyBlobWriter::new(DummyBlob).unwrap()))
                }
                crate::state::Destination::FileSystem { size_bytes, path , mime , uploaded_bytes} => {
                    // let writer: FileSystemBlobBufWriter;

                    // BlobWriter::FileSystem(writer)
                    Err(SessionCreateError::Unknown)
                }
                crate::state::Destination::GoogleCloud {
                    bucket_name,
                    object_name,
                    size_bytes,
                    mime,
                    session_url,
                    uploaded_bytes,
                } => {
                    let writer_result: Result<GoogleCloudBlobWriter, SessionCreateError> = {
                        let bucket_name: String = bucket_name.to_owned();
                        let object_name: String = object_name.to_owned();

                        let source_blob = GoogleCloudBlob::new(bucket_name, object_name);
                        let session_url: Result<url::Url, url::ParseError> =
                            url::Url::parse(session_url);
                        let content_mime_type: String = mime.to_owned();
                        let blob_size: u64 = *size_bytes;
                        let uploaded_bytes: u64 = *uploaded_bytes;

                        if session_url.is_err() {
                            //TODO session_url.unwrap_err();
                            return Err(SessionCreateError::Unknown);
                        }

                        Ok(GoogleCloudBlobWriter::new(
                            source_blob,
                            content_mime_type,
                            blob_size.into(),
                            session_url.unwrap(),
                            uploaded_bytes.into(),
                        ))
                    };

                    if writer_result.is_err() {
                        return Err(writer_result.unwrap_err());
                    }

                    Ok(BlobWriter::GoogleCloud(writer_result.unwrap()))
                }
            }
        };

        if source_reader.is_err() {
            return Err(source_reader.unwrap_err());
        }
        if destination_writer.is_err() {
            return Err(destination_writer.unwrap_err());
        }

        let session = Session {
            source_reader: source_reader.unwrap(),
            destination_writer: destination_writer.unwrap(),
        };

        Ok(session)
    }

    pub fn process(&mut self, buffer_size: u32) -> Result<(), SessionProcessError> {
        // std::io::copy(&mut self.source_reader, &mut self.destination_writer).unwrap();
        // return Ok(());

        let buffer_size: usize = match buffer_size.try_into() {
            Err(_) => {
                return Err(SessionProcessError::Internal(String::from(
                    "Wrong buffer_size value",
                )))
            }
            Ok(v) => v,
        };

        let mut buf = vec![0u8; buffer_size];

        let source_blob_reader = &mut self.source_reader;

        let source_len: u64 = source_blob_reader.len().into();

        let dest_blob_writer = &mut self.destination_writer;

        let start_pos: BlobSize = dest_blob_writer.pos();

        if let Err(err) = source_blob_reader.seek(start_pos) {
            return Err(SessionProcessError::Process(err.to_string()));
        }

        let mut total_read: u64 = start_pos.into();
        let mut tmp_counter = 100;
        loop {
            let read_size: usize = match source_blob_reader.read(&mut buf) {
                Err(err) => {
                    return Err(SessionProcessError::Process(err.to_string()));
                }
                Ok(v) => v,
            };
            if read_size == 0 {
                break;
            }

            let write_size: usize = match dest_blob_writer.write(&buf[..read_size]) {
                Err(err) => {
                    return Err(SessionProcessError::Process(err.to_string()));
                }
                Ok(v) => v,
            };

            if read_size != write_size {
                return Err(SessionProcessError::Process(String::from(
                    "Write size is smaller tha read size. Cannot continue.",
                )));
            }

            let friendly_write_size: u64 = match write_size.try_into() {
                Err(_) => {
                    return Err(SessionProcessError::Internal(String::from(
                        "Unable to convert write size to u64",
                    )));
                }
                Ok(t) => t,
            };

            total_read += friendly_write_size;

            println!(
                "{}/{} (write chunk {} bytes)",
                total_read, source_len, friendly_write_size
            );

            // if --tmp_counter == 0 {
            //     return Err(SessionProcessError::Internal(String::from(
            //         "Temp counter is empty",
            //     )));
            // }
        }
        Ok(())
    }

    pub fn to_state(&self) -> crate::state::SessionState {
        let source: crate::state::Source = match &self.source_reader {
            BlobReader::Dummy(b) => crate::state::Source::from(b),
            BlobReader::FileSystem(b) => crate::state::Source::from(b),
            BlobReader::GoogleCloud(b) => crate::state::Source::from(b),
        };
        let destination: crate::state::Destination = match &self.destination_writer {
            BlobWriter::Dummy(b) => crate::state::Destination::from(b),
            BlobWriter::FileSystem(b) => crate::state::Destination::from(b),
            BlobWriter::GoogleCloud(b) => crate::state::Destination::from(b),
        };
        let state = crate::state::SessionState {
            source: crate::state::Extra { extra: source },
            destination: crate::state::Extra { extra: destination },
        };

        state
    }
}

#[derive(Debug)]
pub enum SessionCreateError {
    Unknown,
}

#[derive(Debug)]
pub enum SessionProcessError {
    Internal(String),
    Process(String),
    Unknown,
}
