mod settings;

use continuously_backup_lib::{
    blob_file_system::FileSystemBlob,
    blob_google_cloud::GoogleCloudBlob,
    session::Session,
    storage::{BlobRead, BlobReader, BlobWrite, BlobWriter},
    storage_file_system::{FileSystemBlobBufReader, FileSystemBlobBufWriter},
    storage_google_cloud::{
        GoogleCloudBlobBufReader, GoogleCloudBlobBufWriter, GoogleCloudPlatformServiceAccount,
    },
};
use settings::{
    CommandCopySession, CommandCopySessionCreateDestination, CommandCopySessionCreateSource,
};

fn main() {
    let command = settings::parse();
    match command {
        settings::Command::CopySession(command_copy_session) => match command_copy_session {
            settings::CommandCopySession::Create {
                buffer_size,
                source,
                destination,
            } => {
                App::command_copy_session_create(buffer_size, &source, &destination);
            }
            settings::CommandCopySession::List {
                session_storage_url,
            } => {
                println!("{:?}", session_storage_url);
            }
            settings::CommandCopySession::Resume {
                session_id,
                session_storage_url,
            } => {
                println!("{:?}", session_id);
                println!("{:?}", session_storage_url);
            }
        },
        settings::Command::Snapshot(command_snapshot) => {
            // TBD
            settings::print_usage_and_exit()
        }
        settings::Command::Nope => settings::print_usage_and_exit(),
    }

    // continuously_backup_lib::session::Session {
    //     content_mime_type: String::from("application/octet-stream"),
    //     source: continuously_backup_lib::session::Extra {
    //         extra: continuously_backup_lib::session::Source::FileSystem {
    //             mime_type: String::from("tttt"),
    //         },
    //     },
    //     destination: continuously_backup_lib::session::Extra {
    //         extra: continuously_backup_lib::session::Destination::GoogleCloud { child_count: 42 },
    //     },
    // };
    // session.dbg();

    // let gcp_service_account: GoogleCloudPlatformServiceAccount =
    //     GoogleCloudPlatformServiceAccount::from_file(
    //         // "/Users/maxim.anurin/w-osfordev/continuously-backup/absolute-garden-272819-d3d737919552.local.json",
    //         "absolute-garden-272819-d3d737919552.local.json",
    //     )
    //     .unwrap();

    // return;

    // let refresh_token = continuously_backup_lib::storage_google_cloud::generate_jwt_refresh_token(
    //     &gcp_service_account,
    // );

    // let access_token_bundle: continuously_backup_lib::storage_google_cloud::AccessTokenBundle =
    //     continuously_backup_lib::storage_google_cloud::api_get_access_token(&refresh_token)
    //         .unwrap();

    // let session_url =
    //     continuously_backup_lib::storage_google_cloud::api_initiate_resumable_upload_session(
    //         &access_token_bundle.access_token,
    //         gcloud_blob_dest.get_bucket_name(),
    //         gcloud_blob_dest.get_object_name(),
    //         // "application/octet-stream",
    //         "text/plain",
    //     )
    //     .unwrap();

    // let upload_status_1 =
    //     continuously_backup_lib::storage_google_cloud::api_check_status_of_resumable_upload(
    //         &session_url,
    //         // 49,
    //     );

    // continuously_backup_lib::storage_google_cloud::api_upload_chunk(
    //     &session_url,
    //     &[65,66,67,68,69],
    //     0,
    //     49,
    // )
    // .unwrap();

    // let upload_status_2 =
    //     continuously_backup_lib::storage_google_cloud::api_check_status_of_resumable_upload(
    //         &session_url,
    //         // 49,
    //     );

    // continuously_backup_lib::storage_google_cloud::api_cancel_upload(&session_url).unwrap();

    // match upload_status_1 {
    //     Ok(_) => {
    //         //
    //     }
    //     Err(err) => {}
    // };

    // match upload_status_2 {
    //     Ok(_) => {
    //         //
    //     }
    //     Err(err) => {}
    // };
}

struct App;
impl App {
    pub fn command_copy_session_create(
        buffer_size: u32,
        source: &CommandCopySessionCreateSource,
        destination: &CommandCopySessionCreateDestination,
    ) -> () {
        let mut session1: continuously_backup_lib::session::Session = {
            let source_blob_reader: BlobReader = {
                match source {
                    CommandCopySessionCreateSource::File { blob } => {
                        BlobReader::FileSystem(FileSystemBlobBufReader::new(blob.clone()).unwrap())
                    }
                    CommandCopySessionCreateSource::GoogleCloud {
                        blob,
                        service_account_json_file,
                    } => BlobReader::GoogleCloud(
                        GoogleCloudBlobBufReader::new(blob.clone()).unwrap(),
                    ),
                }
            };

            let destination_blob_writer: BlobWriter = {
                match destination {
                    CommandCopySessionCreateDestination::File {
                        blob: destination_blob,
                    } => BlobWriter::FileSystem(
                        FileSystemBlobBufWriter::new(destination_blob.clone()).unwrap(),
                    ),
                    CommandCopySessionCreateDestination::GoogleCloud {
                        blob: destination_blob,
                        mime_type,
                        service_account_json_file,
                    } => {
                        // let now: std::time::SystemTime = std::time::SystemTime::now();
                        // let now_unix: u64 =
                        //     now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                        let gcp_service_account: GoogleCloudPlatformServiceAccount =
                            GoogleCloudPlatformServiceAccount::from_file(
                                // "/Users/maxim.anurin/w-osfordev/continuously-backup/absolute-garden-272819-d3d737919552.local.json",
                                // "absolute-garden-272819-d3d737919552.local.json",
                                service_account_json_file,
                            )
                            .unwrap();

                        let refresh_token =
                        continuously_backup_lib::storage_google_cloud::generate_jwt_refresh_token(
                            &gcp_service_account,
                        );

                        let access_token_bundle: continuously_backup_lib::storage_google_cloud::AccessTokenBundle =
                continuously_backup_lib::storage_google_cloud::api_get_access_token(&refresh_token)
                    .unwrap();
                        let blob_size: u64 = source_blob_reader.len();
                        BlobWriter::GoogleCloud(
                            GoogleCloudBlobBufWriter::new_with_access_token(
                                destination_blob.clone(),
                                // String::from("image/png"),
                                // String::from("text/plain"),
                                // String::from("application/octet-stream"),
                                mime_type.clone(),
                                blob_size,
                                access_token_bundle,
                            )
                            .unwrap(),
                        )
                    }
                }
            };

            continuously_backup_lib::session::Session::new(
                source_blob_reader,
                destination_blob_writer,
            )
        };

        let session1_state = session1.to_state();
        session1_state.println();

        // let mut session2: Session = Session::from_state(&session1_state).unwrap();

        session1.process(buffer_size).unwrap();
    }
}
