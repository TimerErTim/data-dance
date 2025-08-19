use crate::config;
use crate::config::{DataDanceConfiguration, LocalStorageConfig, RemoteStorageConfig, WebConfig};
use crate::jobs::restore::RestoreBackupJob;
use crate::jobs::Job;
use crate::objects::{BackupEntry, BackupHistory, BackupType, CompressionLevel};
use crate::services::data_dest::fake::FakeDestService;
use crate::services::data_dest::DestService;
use crate::services::data_source::fake::FakeSourceService;
use crate::services::data_source::SourceService;
use crate::services::data_tunnel::encoding::EncodingDataTunnel;
use crate::services::data_tunnel::DataTunnel;
use std::io::Cursor;
use std::path::PathBuf;

fn base_config(
    password: Option<&str>,
    compression_level: CompressionLevel,
) -> DataDanceConfiguration {
    DataDanceConfiguration {
        web: WebConfig {
            port: 3000,
            host: "0.0.0.0".to_string(),
        },
        local_storage: LocalStorageConfig {
            source: config::LocalSource::Btrfs {
                snapshots_folder: ".snapshots/".into(),
                source_folder: "export/".into(),
                send_compressed_data: true,
            },
            jobs_folder: "./".into(),
        },
        remote_storage: RemoteStorageConfig {
            dest: config::RemoteDestination::Fake,
            encryption: password.map(|pw| pw.into()),
            compression: compression_level,
        },
    }
}

#[test]
fn restoration_runs_through_history() {
    let config = base_config(None, CompressionLevel::Balanced);
    let fake_source = FakeSourceService::new("restored/".into(), 0);
    let fake_dest = FakeDestService::empty();

    // Prepare encoded remote files matching history
    let mut history = BackupHistory { entries: vec![] };
    let e1 = BackupEntry {
        id: 10,
        parent: None,
        timestamp: 100,
        remote_filename: "2024_01_01_12_00_00.bin".into(),
        local_snapshot: "2024_01_01_12_00_00/".into(),
        backup_type: BackupType::Full,
    };
    let e2 = BackupEntry {
        id: 20,
        parent: Some(10),
        timestamp: 200,
        remote_filename: "2024_01_02_12_00_00.dbin".into(),
        local_snapshot: "2024_01_02_12_00_00/".into(),
        backup_type: BackupType::Incremental,
    };
    history.entries.push(e1.clone());
    history.entries.push(e2.clone());

    let encoder = EncodingDataTunnel {
        compression_level: config.remote_storage.compression,
        encryption_level: config.remote_storage.encryption.clone().into(),
    };

    // Write encoded data for each remote file
    {
        let writer = fake_dest
            .get_backup_writer(e1.remote_filename.clone())
            .unwrap();
        let encoded = {
            let (tx, rx) = std::sync::mpsc::channel();
            encoder
                .transfer(
                    Cursor::new(b"hello world".to_vec()),
                    crate::services::channels::ChannelWriter::new(tx),
                )
                .unwrap();
            rx.iter().collect::<Vec<u8>>()
        };
        use std::io::Write;
        let mut w = writer;
        w.write_all(&encoded).unwrap();
        w.flush().unwrap();
    }
    {
        let writer = fake_dest
            .get_backup_writer(e2.remote_filename.clone())
            .unwrap();
        let encoded = {
            let (tx, rx) = std::sync::mpsc::channel();
            encoder
                .transfer(
                    Cursor::new(b"more data".to_vec()),
                    crate::services::channels::ChannelWriter::new(tx),
                )
                .unwrap();
            rx.iter().collect::<Vec<u8>>()
        };
        use std::io::Write;
        let mut w = writer;
        w.write_all(&encoded).unwrap();
        w.flush().unwrap();
    }

    fake_dest.set_backup_history(history).unwrap();

    // Run restore
    let job = RestoreBackupJob::new(config, 0, Box::new(fake_source), Box::new(fake_dest));
    let result = job.run();
    match result.state {
        crate::objects::job_result::RestoreResultState::Error(err) => {
            panic!("restore failed: {}", err)
        }
        _ => {}
    }
}

fn make_history_chain(ids: &[(u32, Option<u32>)]) -> BackupHistory {
    let mut h = BackupHistory { entries: vec![] };
    let mut ts = 1u64;
    for (i, p) in ids.iter() {
        let name = if p.is_some() {
            format!("{i}.dbin")
        } else {
            format!("{i}.bin")
        };
        h.entries.push(BackupEntry {
            id: *i,
            parent: *p,
            timestamp: ts,
            remote_filename: name.clone().into(),
            local_snapshot: format!("{i}/").into(),
            backup_type: if p.is_some() {
                BackupType::Incremental
            } else {
                BackupType::Full
            },
        });
        ts += 1;
    }
    h
}

fn write_encoded(
    fake_dest: &FakeDestService,
    name: &str,
    bytes: &[u8],
    compression: CompressionLevel,
    password: Option<&str>,
) {
    let encoder = EncodingDataTunnel {
        compression_level: compression,
        encryption_level: password
            .map(|p| crate::objects::SensitiveString::from(p))
            .into(),
    };
    let writer = fake_dest.get_backup_writer(name.into()).unwrap();
    let encoded = {
        let (tx, rx) = std::sync::mpsc::channel();
        encoder
            .transfer(
                Cursor::new(bytes.to_vec()),
                crate::services::channels::ChannelWriter::new(tx),
            )
            .unwrap();
        rx.iter().collect::<Vec<u8>>()
    };
    use std::io::Write;
    let mut w = writer;
    w.write_all(&encoded).unwrap();
    w.flush().unwrap();
}

#[test]
fn e2e_backup_then_restore_latest_and_intermediate() {
    // Prepare config
    let compression = CompressionLevel::Best;
    let password: Option<&str> = Some("pw");
    let mut config = base_config(password, compression);

    // Simulate three backups into fake dest
    let fake_dest = FakeDestService::empty();
    let history = make_history_chain(&[(1, None), (2, Some(1)), (3, Some(2))]);
    write_encoded(&fake_dest, "1.bin", b"v1 full", compression, password);
    write_encoded(&fake_dest, "2.dbin", b"v2 delta", compression, password);
    write_encoded(&fake_dest, "3.dbin", b"v3 delta", compression, password);
    fake_dest.set_backup_history(history.clone()).unwrap();

    // Restore latest (id=3)
    let src = FakeSourceService::new("restored_latest/".into(), 0);
    let job_latest = RestoreBackupJob::new(
        config.clone(),
        3,
        Box::new(src),
        Box::new(fake_dest.clone()),
    );
    let res_latest = job_latest.run();
    if let crate::objects::job_result::RestoreResultState::Error(err) = res_latest.state {
        panic!("restore latest failed: {}", err)
    }

    // Restore to intermediate (id=2)
    let src_mid = FakeSourceService::new("restored_mid/".into(), 0);
    let job_mid = RestoreBackupJob::new(
        config.clone(),
        2,
        Box::new(src_mid),
        Box::new(fake_dest.clone()),
    );
    let res_mid = job_mid.run();
    if let crate::objects::job_result::RestoreResultState::Error(err) = res_mid.state {
        panic!("restore mid failed: {}", err)
    }
}
