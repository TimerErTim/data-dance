import {CompressionLevel} from "@/lib/queries/spec";

export type CurrentBackupJob = {
    startedAt: Date
} & ({
    incremental: {
        stage: CurrentIncrementalBackupFetchingMetadata | CurrentIncrementalBackupUploading
    }
})

export type CurrentIncrementalBackupFetchingMetadata = {
    tag: "FetchingMetadata"
}

export type CurrentIncrementalBackupUploading = {
    tag: "Uploading",
    timestamp: Date,
    parent: number | null,
    remoteFilename: string,
    localSnapshot: string,
    bytesRead: number,
    bytesWritten: number,
    bytesWrittenPerSecond: number,
    compressionLevel: CompressionLevel;
    encrypted: boolean,
    finishing: boolean,
}
