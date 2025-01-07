use continuously_backup_lib::{
    blob::{BlobSize, DummyBlob}, 
    blob_file_system::FileSystemBlob, 
    blob_google_cloud::GoogleCloudBlob,
};

#[derive(Clone, Debug)]
pub enum Command {
    CopySession(CommandCopySession),
    Snapshot(CommandSnapshot),
    Nope,
}

#[derive(Clone, Debug)]
pub enum CommandCopySession {
    Create {
        buffer_size: u32,
        source: CommandCopySessionCreateSource,
        destination: CommandCopySessionCreateDestination,
    },
    List {
        session_storage_url: Option<url::Url>,
    },
    Resume {
        session_id: Option<String>,
        session_storage_url: Option<url::Url>,
    },
}

#[derive(Clone, Debug)]
pub enum CommandCopySessionCreateSource {
    Dummy{
        blob: DummyBlob,
        blob_size: BlobSize,
    },
    File {
        blob: FileSystemBlob,
    },
    GoogleCloud {
        blob: GoogleCloudBlob,
        // mime_type: String,
        service_account_json_file: String,
    },
}

#[derive(Clone, Debug)]
pub enum CommandCopySessionCreateDestination {
    Dummy{
        blob: DummyBlob,
    },
    File {
        blob: FileSystemBlob,
    },
    GoogleCloud {
        blob: GoogleCloudBlob,
        mime_type: String,
        service_account_json_file: String,
    },
}

#[derive(Clone, Debug)]
pub enum CommandSnapshot {
    // TBD
}

pub fn parse() -> Command {
    internal::parse()
}

pub fn print_usage_and_exit() -> ! {
    match internal::print_usage() {
        Ok(()) => {}
        Err(err) => {
            // ??? TODO
        }
    }
    std::process::exit(-1);
}

mod internal {
    use clap::{Args, CommandFactory, Parser, Subcommand};
    use continuously_backup_lib::{
        blob::DummyBlob, blob_file_system::FileSystemBlob, blob_google_cloud::GoogleCloudBlob
    };

    #[derive(Parser)]
    #[command(about, long_about = None)]
    struct Cli {
        #[command(subcommand)]
        command: CliCommand,
    }

    #[derive(Subcommand)]
    enum CliCommand {
        ///
        /// Session to copy a blob between storages
        ///
        CopySession {
            #[command(subcommand)]
            command: CliCommandCopySession,
        },

        ///
        /// Manage snapshots
        ///
        Snapshot(CliCommandSnapshot),
    }

    #[derive(Subcommand)]
    enum CliCommandCopySession {
        ///
        /// Session to copy a blob between storages
        ///
        Create(CliCommandCopySessionCreate),

        ///
        /// List sessions
        ///
        List(CliCommandCopySessionList),

        ///
        /// Resume a session
        ///
        Resume(CliCommandCopySessionResume),
    }

    #[derive(Clone, Debug, Args)]
    struct CliCommandCopySessionCreateSource {
        #[arg(
            long,
            conflicts_with("source_file"),
            conflicts_with("source_google_cloud_bucket_name"),
            conflicts_with("source_google_cloud_object_name"),
            conflicts_with("source_google_cloud_mime_type"),
            conflicts_with("source_google_cloud_service_account_json_file")
        )]
        pub source_dummy_size: Option<u64>,

        #[arg(
            long,
            conflicts_with("source_dummy_size"),
            conflicts_with("source_google_cloud_bucket_name"),
            conflicts_with("source_google_cloud_object_name"),
            conflicts_with("source_google_cloud_mime_type"),
            conflicts_with("source_google_cloud_service_account_json_file")
        )]
        source_file: Option<String>,

        #[arg(
            long,
            conflicts_with("source_dummy_size"),
            conflicts_with("source_file"),
            requires("source_google_cloud_object_name"),
            requires("source_google_cloud_mime_type"),
            requires("source_google_cloud_service_account_json_file")
        )]
        source_google_cloud_bucket_name: Option<String>,

        #[arg(
            long,
            conflicts_with("source_dummy_size"),
            conflicts_with("source_file"),
            requires("source_google_cloud_bucket_name"),
            requires("source_google_cloud_mime_type"),
            requires("source_google_cloud_service_account_json_file")
        )]
        source_google_cloud_object_name: Option<String>,

        #[arg(
            long,
            conflicts_with("source_dummy_size"),
            conflicts_with("source_file"),
            requires("source_google_cloud_bucket_name"),
            requires("source_google_cloud_object_name"),
            requires("source_google_cloud_service_account_json_file")
        )]
        source_google_cloud_mime_type: Option<String>,

        #[arg(
            long,
            conflicts_with("source_dummy_size"),
            conflicts_with("source_file"),
            requires("source_google_cloud_bucket_name"),
            requires("source_google_cloud_object_name"),
            requires("source_google_cloud_mime_type")
        )]
        source_google_cloud_service_account_json_file: Option<String>,
    }

    #[derive(Clone, Debug, Args)]
    struct CliCommandCopySessionCreateDestination {
        #[clap(
            long,
            action,
            conflicts_with("destination_file"),
            conflicts_with("destination_google_cloud_bucket_name"),
            conflicts_with("destination_google_cloud_object_name"),
            conflicts_with("destination_google_cloud_mime_type"),
            conflicts_with("destination_google_cloud_service_account_json_file")
        )]
        pub destination_dummy: bool,

        #[arg(
            long,
            conflicts_with("destination_dummy"),
            conflicts_with("destination_google_cloud_bucket_name"),
            conflicts_with("destination_google_cloud_object_name"),
            conflicts_with("destination_google_cloud_mime_type"),
            conflicts_with("destination_google_cloud_service_account_json_file")
        )]
        pub destination_file: Option<String>,

        #[arg(
            long,
            conflicts_with("destination_dummy"),
            conflicts_with("destination_file"),
            requires("destination_google_cloud_object_name"),
            requires("destination_google_cloud_mime_type"),
            requires("destination_google_cloud_service_account_json_file")
        )]
        pub destination_google_cloud_bucket_name: Option<String>,

        #[arg(
            long,
            conflicts_with("destination_dummy"),
            conflicts_with("destination_file"),
            requires("destination_google_cloud_bucket_name"),
            requires("destination_google_cloud_mime_type"),
            requires("destination_google_cloud_service_account_json_file")
        )]
        pub destination_google_cloud_object_name: Option<String>,

        #[arg(
            long,
            conflicts_with("destination_dummy"),
            conflicts_with("destination_file"),
            requires("destination_google_cloud_bucket_name"),
            requires("destination_google_cloud_object_name"),
            requires("destination_google_cloud_service_account_json_file")
        )]
        pub destination_google_cloud_mime_type: Option<String>,

        #[arg(
            long,
            conflicts_with("destination_dummy"),
            conflicts_with("destination_file"),
            requires("destination_google_cloud_bucket_name"),
            requires("destination_google_cloud_object_name"),
            requires("destination_google_cloud_mime_type")
        )]
        pub destination_google_cloud_service_account_json_file: Option<String>,
    }

    #[derive(Args)]
    struct CliCommandCopySessionCreate {
        #[arg(
            long, 
            default_value = "8388608", // 8MB
            )]
        buffer_size: Option<u32>,

        #[arg(long)]
        session_storage_url: Option<String>,

        #[clap(flatten)]
        source: CliCommandCopySessionCreateSource,

        #[clap(flatten)]
        destination: CliCommandCopySessionCreateDestination,
    }

    #[derive(Args)]
    pub struct CliCommandCopySessionList {
        #[arg(long)]
        pub session_storage_url: Option<String>,
    }

    #[derive(Args)]
    pub struct CliCommandCopySessionResume {
        #[arg(long)]
        pub session_id: Option<String>,

        #[arg(long)]
        pub session_storage_url: Option<String>,
    }

    #[derive(Args)]
    pub struct CliCommandSnapshot {
        /// Name of the person to greet
        #[arg(short, long)]
        pub volume_group: String,

        /// Number of times to greet
        #[arg(short, long)]
        pub logical_volume_name: String,
    }

    pub fn print_usage() -> std::io::Result<()> {
        Cli::command().print_help()
    }

    pub fn parse() -> super::Command {
        match Cli::parse().command {
            CliCommand::CopySession {
                command: cmd_copy_session,
            } => {
                super::Command::CopySession(match cmd_copy_session {
                    CliCommandCopySession::Create(cmd_copy_session_create) => {
                        let source: &CliCommandCopySessionCreateSource =
                            &cmd_copy_session_create.source;
                        let destination: &CliCommandCopySessionCreateDestination =
                            &cmd_copy_session_create.destination;

                        super::CommandCopySession::Create {
                            buffer_size: cmd_copy_session_create.buffer_size.unwrap(),
                            source: {
                                if source.source_dummy_size.is_some() {
                                    super::CommandCopySessionCreateSource::Dummy {
                                        blob: DummyBlob,
                                        blob_size:source.source_dummy_size.clone().unwrap().into(),
                                    }
                                } else if source.source_file.is_some() {
                                    super::CommandCopySessionCreateSource::File {
                                        blob: FileSystemBlob::new(
                                            source.source_file.clone().unwrap(),
                                        ),
                                    }
                                }  else
                                // TODO
                                // if (source.source_google_cloud_bucket_name.is_some()
                                //     && source.source_google_cloud_mime_type.is_some()
                                //     && source.source_google_cloud_object_name.is_some()
                                //     && source
                                //         .source_google_cloud_service_account_json_file
                                //         .is_some())
                                {
                                    super::CommandCopySessionCreateSource::GoogleCloud {
                                        blob: GoogleCloudBlob::new(
                                            source.source_google_cloud_bucket_name.clone().unwrap(),
                                            source.source_google_cloud_object_name.clone().unwrap(),
                                        ),
                                        // mime_type: source.source_google_cloud_mime_type.unwrap(),
                                        service_account_json_file: source
                                            .source_google_cloud_service_account_json_file
                                            .clone()
                                            .unwrap(),
                                    }
                                }
                            },
                            destination: {
                                if destination.destination_dummy {
                                    super::CommandCopySessionCreateDestination::Dummy {
                                        blob: DummyBlob,
                                    }
                                } else if destination.destination_file.is_some() {
                                    super::CommandCopySessionCreateDestination::File {
                                        blob: FileSystemBlob::new(
                                            destination.destination_file.clone().unwrap(),
                                        ),
                                    }
                                } else
                                // TODO
                                // if (source.source_google_cloud_bucket_name.is_some()
                                //     && source.source_google_cloud_mime_type.is_some()
                                //     && source.source_google_cloud_object_name.is_some()
                                //     && source
                                //         .source_google_cloud_service_account_json_file
                                //         .is_some())
                                {
                                    super::CommandCopySessionCreateDestination::GoogleCloud {
                                        blob: GoogleCloudBlob::new(
                                            destination
                                                .destination_google_cloud_bucket_name
                                                .clone()
                                                .unwrap(),
                                            destination
                                                .destination_google_cloud_object_name
                                                .clone()
                                                .unwrap(),
                                        ),
                                        mime_type: destination
                                            .destination_google_cloud_mime_type
                                            .clone()
                                            .unwrap(),
                                        service_account_json_file: destination
                                            .destination_google_cloud_service_account_json_file
                                            .clone()
                                            .unwrap(),
                                    }
                                }
                            },
                        }
                    }
                    CliCommandCopySession::List(cmd_copy_session_list) => {
                        super::CommandCopySession::List {
                            session_storage_url: cmd_copy_session_list
                                .session_storage_url
                                .map(|s| url::Url::parse(&s).unwrap()),
                        }
                    }
                    CliCommandCopySession::Resume(cmd_copy_session_resume) => {
                        super::CommandCopySession::Resume {
                            session_id: cmd_copy_session_resume.session_id.clone(),
                            session_storage_url: cmd_copy_session_resume
                                .session_storage_url
                                .map(|s| url::Url::parse(&s).unwrap()),
                        }
                    }
                })
            }
            CliCommand::Snapshot(cmd_snapshot) => super::Command::Nope,
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn should_parse_copy_session_create_file_to_gcloud() {
            let cli_result: Result<Cli, clap::error::Error> = Cli::try_parse_from([
                "no_matter_app_name",
                "copy-session",
                "create",
                "--buffer-size=524288",
                "--source-file=/var/tmp/image.png",
                "--destination-google-cloud-bucket-name=backups",
                "--destination-google-cloud-object-name=debug/image-20250105-01.png",
                "--destination-google-cloud-mime-type=image/png",
                "--destination-google-cloud-service-account-json-file=/etc/continuously/absolute-garden-272819-d3d737919552.local.json",
                ]);

            assert!(cli_result.is_ok());

            let cli: Cli = cli_result.unwrap();

            if let CliCommand::CopySession {
                command: cli_cmd_copy_session,
            } = cli.command
            {
                if let CliCommandCopySession::Create(cli_cmd_copy_session_create) =
                    cli_cmd_copy_session
                {
                    assert!(cli_cmd_copy_session_create.session_storage_url.is_none());

                    assert!(cli_cmd_copy_session_create.buffer_size.unwrap() == 524288);

                    let source: &CliCommandCopySessionCreateSource =
                        &cli_cmd_copy_session_create.source;
                    let destination: &CliCommandCopySessionCreateDestination =
                        &cli_cmd_copy_session_create.destination;

                    assert!(source.source_dummy_size.is_none());
                    assert!(source.source_file.is_some());
                    assert!(source.source_google_cloud_bucket_name.is_none());
                    assert!(source.source_google_cloud_mime_type.is_none());
                    assert!(source.source_google_cloud_object_name.is_none());
                    assert!(source
                        .source_google_cloud_service_account_json_file
                        .is_none());

                    assert!(destination.destination_dummy == false);
                    assert!(destination.destination_file.is_none());
                    assert!(destination.destination_google_cloud_bucket_name.is_some());
                    assert!(destination.destination_google_cloud_mime_type.is_some());
                    assert!(destination.destination_google_cloud_object_name.is_some());
                    assert!(destination
                        .destination_google_cloud_service_account_json_file
                        .is_some());

                    assert!(source.source_file.as_ref().unwrap() == "/var/tmp/image.png");

                    assert!(
                        destination
                            .destination_google_cloud_bucket_name
                            .as_ref()
                            .unwrap()
                            == "backups"
                    );
                    assert!(
                        destination
                            .destination_google_cloud_object_name
                            .as_ref()
                            .unwrap()
                            == "debug/image-20250105-01.png"
                    );
                    assert!(
                        destination
                            .destination_google_cloud_mime_type
                            .as_ref()
                            .unwrap()
                            == "image/png"
                    );
                    assert!(
                        destination
                            .destination_google_cloud_service_account_json_file
                            .as_ref()
                            .unwrap()
                            == "/etc/continuously/absolute-garden-272819-d3d737919552.local.json"
                    );
                } else {
                    panic!("CliCommandCopySessionCreate was not parsed");
                }
            } else {
                panic!("CliCommandCopySession was not parsed");
            }
        }

        #[test]
        fn should_parse_copy_session_create_file_to_dummy() {
            let cli_result: Result<Cli, clap::error::Error> = Cli::try_parse_from([
                "no_matter_app_name",
                "copy-session",
                "create",
                "--source-file=/var/tmp/image.png",
                "--destination-dummy",
                ]);

            assert!(cli_result.is_ok());

            let cli: Cli = cli_result.unwrap();

            if let CliCommand::CopySession {
                command: cli_cmd_copy_session,
            } = cli.command
            {
                if let CliCommandCopySession::Create(cli_cmd_copy_session_create) =
                    cli_cmd_copy_session
                {
                    assert!(cli_cmd_copy_session_create.session_storage_url.is_none());

                    let source: &CliCommandCopySessionCreateSource =
                        &cli_cmd_copy_session_create.source;
                    let destination: &CliCommandCopySessionCreateDestination =
                        &cli_cmd_copy_session_create.destination;

                    assert!(source.source_dummy_size.is_none());
                    assert!(source.source_file.is_some());
                    assert!(source.source_google_cloud_bucket_name.is_none());
                    assert!(source.source_google_cloud_mime_type.is_none());
                    assert!(source.source_google_cloud_object_name.is_none());
                    assert!(source
                        .source_google_cloud_service_account_json_file
                        .is_none());

                    assert!(destination.destination_dummy == true);
                    assert!(destination.destination_file.is_none());
                    assert!(destination.destination_google_cloud_bucket_name.is_none());
                    assert!(destination.destination_google_cloud_mime_type.is_none());
                    assert!(destination.destination_google_cloud_object_name.is_none());
                    assert!(destination
                        .destination_google_cloud_service_account_json_file
                        .is_none());

                    assert!(source.source_file.as_ref().unwrap() == "/var/tmp/image.png");

                } else {
                    panic!("CliCommandCopySessionCreate was not parsed");
                }
            } else {
                panic!("CliCommandCopySession was not parsed");
            }
        }
    }
}
