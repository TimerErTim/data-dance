export type CompressionLevel = "None" | "Fast" | "Balanced" | "Best";

export type CurrentJobsAPI = {
    restore: never,
    backup: null | CurrentBackupJob
}

type CurrentBackupJob = {
    Incremental: CurrentIncrementalBackupJob
}

type CurrentIncrementalBackupJob = {
    started_at: string,
    stage: CurrentIncrementalBackupJobStage
}

type CurrentIncrementalBackupJobStage = {
    Uploading: {
        timestamp: string,
        parent: number | null,
        remote_filename: string,
        local_snapshot: string,
        bytes_read: number,
        bytes_written: number,
        compression_level: CompressionLevel;
        encrypted: boolean,
        finishing: boolean,
    },
    FetchingMetadata: never,
} | "FetchingMetadata"
