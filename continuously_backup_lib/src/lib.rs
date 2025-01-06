pub mod blob;
pub mod blob_file_system;
pub mod blob_google_cloud;
pub mod session;
mod state;
pub mod storage;
pub mod storage_file_system;
pub mod storage_google_cloud;

// pub trait BlobOperation {
//     fn open_reader(&self) -> std::io::Result<Box<dyn BlobReader>>;
//     fn open_writer(&self) -> std::io::Result<Box<dyn BlobWriter>>;
// }

trait Stateable<T> {
    fn to_state(&self) -> T;
}

// trait SourceState {
//     fn to_state(&self) -> state::Source;
// }

// trait DestinationState {
//     fn to_state(&self) -> state::Destination;
// }




