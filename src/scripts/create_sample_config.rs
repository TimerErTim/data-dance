use crate::config::{
    DataDanceConfiguration, LocalSource, LocalStorageConfig, RemoteDestination,
    RemoteStorageConfig, WebConfig,
};
use crate::objects::CompressionLevel;
use std::path::PathBuf;

fn make_sample_config() -> DataDanceConfiguration {
    DataDanceConfiguration {
        web: WebConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
        },
        local_storage: LocalStorageConfig {
            source: LocalSource::Btrfs {
                snapshots_folder: PathBuf::from("/mnt/mstrg/backups/.snapshots/"),
                source_folder: PathBuf::from("/mnt/mstrg/export/"),
                send_compressed_data: true,
            },
            jobs_folder: PathBuf::from("/mnt/mstrg/backups/"),
        },
        remote_storage: RemoteStorageConfig {
            dest: RemoteDestination::Ssh {
                username: "u428321".to_string(),
                hostname: "u428321.your-storagebox.de".to_string(),
                port: Some(23),
                folder: "/home/chaotix".into(),
            },
            encryption: Some("123456".into()),
            compression: CompressionLevel::Best,
        },
    }
}

#[test]
fn print_sample_config() {
    let config = make_sample_config();
    let config_toml = toml::ser::to_string(&config).unwrap();

    print!("{}", config_toml);
}
