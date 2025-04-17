import {CompressionLevel} from "@/lib/queries/spec";
import {REnum} from "@/lib/types";

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

export type HistoryBackupJob = {
    startedAt: Date,
    finishedAt: Date,
    result: REnum<{
        Error: string,
        Success: {
            id: number,
            parent: number | null,
            remoteFilename: string,
            localSnapshot: string,
            bytesRead: number,
            bytesWritten: number,
            compressionLevel: CompressionLevel,
            encrypted: boolean
        }
    }>
}

type Result = REnum<{
    Error: string,
    Success: {
        id: number,
        parent: number | null,
        remoteFilename: string,
        localSnapshot: string,
        bytesRead: number,
        bytesWritten: number,
        compressionLevel: CompressionLevel,
        encrypted: boolean
    }
}>
