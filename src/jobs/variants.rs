use crate::jobs::incremental_backup::IncrementalBackupJob;
use crate::jobs::Job;

pub enum JobVariant {
    Restoration(RestorationJobVariant),
    Backup(BackupJobVariant),
}

pub enum RestorationJobVariant {
    DataRestoration(),
}

pub enum BackupJobVariant {
    FullDataBackup(),
    IncrementalDataBackup(IncrementalBackupJob),
}

impl From<IncrementalBackupJob> for JobVariant {
    fn from(value: IncrementalBackupJob) -> Self {
        JobVariant::Backup(BackupJobVariant::IncrementalDataBackup(value))
    }
}
