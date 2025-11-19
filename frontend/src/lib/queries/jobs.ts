import {queryOptions, useQuery} from "@tanstack/react-query";
import config from "@/lib/config";
import {CurrentJobsAPI} from "@/lib/queries/spec";
import {useEffect, useState} from "react";
import {
    CurrentBackupJob,
    CurrentIncrementalBackupFetchingMetadata,
    CurrentIncrementalBackupUploading
} from "@/lib/model";

export function convertCurrentBackupJob(data: CurrentJobsAPI): CurrentBackupJob | null {
    if (!data.backup) {
        return null
    }

    const stage: CurrentIncrementalBackupUploading | CurrentIncrementalBackupFetchingMetadata = (() => {
        if (data.backup!!.Incremental.stage === "FetchingMetadata") {
            return {
                tag: "FetchingMetadata"
            }
        } else {
            const uploading = data.backup!!.Incremental.stage.Uploading
            return {
                tag: "Uploading",
                timestamp: new Date(uploading.timestamp),
                parent: uploading.parent,
                remoteFilename: uploading.remote_filename,
                localSnapshot: uploading.local_snapshot,
                bytesRead: uploading.bytes_read,
                bytesWritten: uploading.bytes_written,
                bytesWrittenPerSecond: 0,
                compressionLevel: uploading.compression_level,
                encrypted: uploading.encrypted,
                finishing: uploading.finishing
            }
        }
    })()

    return {
        startedAt: new Date(data.backup.Incremental.started_at),
        incremental: {
            stage
        }
    }
}

export function currentJobsQuery() {
    return queryOptions({
        queryKey: ['currentJobs'],
        queryFn: async () => {
            const response = await fetch(config.host + '/api/jobs/status');
            const payload = await response.json();
            return payload as CurrentJobsAPI
        },
        refetchInterval: 1000
    })
}

export function useCurrentBackupJob() {
    const query = useQuery(currentJobsQuery())
    const [previousQueries, setPreviousQueries] = useState([] as CurrentBackupJob[])

    useEffect(() => {
        if (!query.data) {
            return
        }

        const backupJob = convertCurrentBackupJob(query.data)
        if (!backupJob || backupJob.incremental.stage.tag !== "Uploading") {
            previousQueries.length = 0
            return
        }

        previousQueries.push(backupJob)
        if (previousQueries.length > 5) {
            previousQueries.shift()
        }

        const previousQuery = previousQueries[previousQueries.length - 1]
        if (!previousQuery) {
            return
        }
        const previousBytesWritten = previousQuery.incremental.stage.tag === "Uploading" ? previousQuery.incremental.stage.bytesWritten : 0
        if (previousBytesWritten > backupJob.incremental.stage.bytesWritten) {
            previousQueries.length = 0
        }
    }, [query.data]);

    if (query.data) {
        const newData = convertCurrentBackupJob(query.data)

        const previousBackupJobs = previousQueries.filter((job) => job.incremental.stage.tag === "Uploading")
        if (previousBackupJobs.length === 0 || !newData) {
            return {
                ...query,
                data: newData,
            }
        }

        const referenceQuery = previousBackupJobs[0]
        if (!referenceQuery) {
            return {
                ...query,
                data: newData
            }
        }
        if (referenceQuery.incremental.stage.tag !== "Uploading" || newData.incremental.stage.tag !== "Uploading" || newData.incremental.stage.timestamp.getTime() == referenceQuery.incremental.stage.timestamp.getTime()) {
            return {
                ...query,
                data: newData
            }
        }

        const referenceBytesWritten = referenceQuery.incremental.stage.bytesWritten
        const currentBytesWritten = newData.incremental.stage.bytesWritten
        const bytesDifference = currentBytesWritten - referenceBytesWritten
        const duration = (newData.incremental.stage.timestamp.getTime() - referenceQuery.incremental.stage.timestamp.getTime()) / 1000


        const bytesWrittenPerSecond = bytesDifference / duration
        console.log("bytesWrittenPerSecond", bytesWrittenPerSecond)

        if (newData.incremental.stage.tag === "Uploading") {
            newData.incremental.stage.bytesWrittenPerSecond = bytesWrittenPerSecond
        }
        return {
            ...query,
            data: newData
        }
    }
    return query
}
