use crate::config::{DataDanceConfiguration, LocalStorageConfig, RemoteStorageConfig, WebConfig};
use crate::objects::CompressionLevel;
use std::path::PathBuf;

fn make_sample_config() -> DataDanceConfiguration {
    DataDanceConfiguration {
        web: WebConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
        },
        local_storage: LocalStorageConfig {
            snapshots_folder: PathBuf::from("/mnt/mstrg/backups/.snapshots/"),
            source_folder: PathBuf::from("/mnt/mstrg/export/"),
            jobs_folder: PathBuf::from("/mnt/mstrg/backups/"),
        },
        remote_storage: RemoteStorageConfig {
            dest_folder: "/chaotix/nas/backups/".into(),
            password: Some("123456".into()),
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
