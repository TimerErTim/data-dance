use crate::jobs::incremental_backup::IncrementalBackupJob;
use crate::jobs::restore::RestoreBackupJob;
use crate::jobs::Job;

pub enum JobVariant {
    Restoration(RestorationJobVariant),
    Backup(BackupJobVariant),
}

pub enum RestorationJobVariant {
    DataRestoration(RestoreBackupJob),
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

impl From<RestoreBackupJob> for JobVariant {
    fn from(value: RestoreBackupJob) -> Self {
        JobVariant::Restoration(RestorationJobVariant::DataRestoration(value))
    }
}
