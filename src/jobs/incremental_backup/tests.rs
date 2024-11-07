use crate::config::{DataDanceConfiguration, LocalStorageConfig, RemoteStorageConfig, WebConfig};
use crate::jobs::incremental_backup::IncrementalBackupJob;
use crate::jobs::Job;
use crate::objects::job_result::{IncrementalBackupResult, IncrementalBackupResultState};
use crate::objects::{BackupEntry, BackupHistory, BackupType, CompressionLevel};
use crate::services::data_dest::fake::FakeDestService;
use crate::services::data_dest::DestService;
use crate::services::data_source::fake::FakeSourceService;
use crate::services::data_source::SourceService;
use crate::{config, objects};
use std::cell::RefCell;
use std::path::PathBuf;

fn run_job(
    config: DataDanceConfiguration,
    source: impl SourceService + Send + 'static,
    dest: impl DestService + Send + 'static,
) -> IncrementalBackupResult {
    let job = IncrementalBackupJob::new(config, Box::new(source), Box::new(dest));
    let result = job.run();
    result
}

fn run_fake_job(
    history: BackupHistory,
    new_local_snapshot: &str,
    password: Option<&str>,
    compression_level: CompressionLevel,
    source_bytes_count: usize,
) -> IncrementalBackupTestData {
    let config = DataDanceConfiguration {
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
            dest: config::RemoteDestination::Local {
                folder: "backups/".into(),
            },
            encryption: password.map(|pw| pw.into()),
            compression: compression_level,
        },
    };

    let fake_source = FakeSourceService::new(new_local_snapshot.into(), source_bytes_count);
    let fake_dest = FakeDestService::new(history);

    let fake_source_debug = fake_source.live_debug_data();
    let fake_dest_debug = fake_dest.live_debug_data();

    let result = run_job(config, fake_source, fake_dest);

    IncrementalBackupTestData {
        run_result: result,
        local_snapshots_clear: fake_source_debug.local_snapshots_cleared(),
        stored_backup_history: fake_dest_debug.history(),
    }
}

struct IncrementalBackupTestData {
    run_result: IncrementalBackupResult,
    local_snapshots_clear: bool,
    stored_backup_history: BackupHistory,
}

#[test]
fn incremental_backup_initial_full() {
    let test_data = run_fake_job(
        BackupHistory { entries: vec![] },
        "2024_01_01_12_00_00/",
        Some("123456"),
        CompressionLevel::Best,
        100 * 1024 * 1024,
    );
    let result = test_data.run_result;

    match &result.state {
        IncrementalBackupResultState::Error(_) => panic!("Job errored"),
        IncrementalBackupResultState::Success(result) => {
            assert_eq!(result.compression_level, CompressionLevel::Best);
            assert_eq!(result.encrypted, true);
            assert_eq!(result.bytes_read, 100 * 1024 * 1024);
            assert_eq!(result.parent, None);
            assert_eq!(result.local_snapshot, "2024_01_01_12_00_00/");
            assert_eq!(result.remote_filename, "2024_01_01_12_00_00.bin");
        }
    }
    assert_eq!(test_data.local_snapshots_clear, true);
    let latest_history_entry = test_data.stored_backup_history.entries.last().unwrap();
    assert_eq!(latest_history_entry.backup_type, BackupType::Full);
    assert_eq!(latest_history_entry.parent, None);
    assert_eq!(
        latest_history_entry.local_snapshot,
        PathBuf::from("2024_01_01_12_00_00/")
    );
    assert_eq!(
        latest_history_entry.remote_filename,
        PathBuf::from("2024_01_01_12_00_00.bin")
    );
}

#[test]
fn incremental_backup_with_parent() {
    let test_data = run_fake_job(
        BackupHistory {
            entries: vec![
                BackupEntry {
                    id: 10,
                    parent: None,
                    timestamp: 100,
                    remote_filename: "2024_01_01_12_00_00.bin".into(),
                    local_snapshot: "2024_01_01_12_00_00/".into(),
                    backup_type: BackupType::Full,
                },
                BackupEntry {
                    id: 20,
                    parent: Some(10),
                    timestamp: 200,
                    remote_filename: "2024_01_02_12_00_00.dbin".into(),
                    local_snapshot: "2024_01_02_12_00_00/".into(),
                    backup_type: BackupType::Incremental,
                },
            ],
        },
        "2024_01_03_12_00_00/",
        Some("123456"),
        CompressionLevel::Best,
        10 * 1024 * 1024,
    );
    let result = test_data.run_result;

    match &result.state {
        IncrementalBackupResultState::Error(_) => panic!("Job errored"),
        IncrementalBackupResultState::Success(result) => {
            assert_eq!(result.bytes_read, 10 * 1024 * 1024);
            assert_eq!(result.parent, Some(20));
            assert_eq!(result.local_snapshot, "2024_01_03_12_00_00/");
            assert_eq!(result.remote_filename, "2024_01_03_12_00_00.dbin");
        }
    }
    assert_eq!(test_data.local_snapshots_clear, true);
    let latest_history_entry = test_data.stored_backup_history.entries.last().unwrap();
    assert_eq!(latest_history_entry.backup_type, BackupType::Incremental);
    assert_eq!(latest_history_entry.parent, Some(20));
    assert_eq!(
        latest_history_entry.local_snapshot,
        PathBuf::from("2024_01_03_12_00_00/")
    );
    assert_eq!(
        latest_history_entry.remote_filename,
        PathBuf::from("2024_01_03_12_00_00.dbin")
    );
}

#[test]
fn incremental_backup_encryption() {
    let test_data = run_fake_job(
        BackupHistory { entries: vec![] },
        "2024_01_01/",
        Some("123456"),
        CompressionLevel::None,
        10 * 1024 * 1024,
    );
    let result = test_data.run_result;

    match &result.state {
        IncrementalBackupResultState::Error(_) => panic!("Job errored"),
        IncrementalBackupResultState::Success(result) => {
            assert_eq!(result.encrypted, true);
            assert_eq!(result.compression_level, CompressionLevel::None);
        }
    }
}

#[test]
fn incremental_backup_compression() {
    let test_data = run_fake_job(
        BackupHistory { entries: vec![] },
        "2024_01_01/",
        None,
        CompressionLevel::Balanced,
        10 * 1024 * 1024,
    );
    let result = test_data.run_result;

    match &result.state {
        IncrementalBackupResultState::Error(_) => panic!("Job errored"),
        IncrementalBackupResultState::Success(result) => {
            assert_eq!(result.encrypted, false);
            assert_eq!(result.compression_level, CompressionLevel::Balanced);
        }
    }
}
