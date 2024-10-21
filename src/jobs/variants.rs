use crate::jobs::Job;

pub enum JobVariant {
    FullDataBackup(),
    IncrementalDataBackup(),
    DataRestoration(),
}
