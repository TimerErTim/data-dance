import {queryOptions, useQuery} from "@tanstack/react-query";
import config from "@/lib/config";
import {CurrentJobsAPI} from "@/lib/queries/spec";
import {useEffect, useState} from "react";
import {
    CurrentBackupJob,
    CurrentIncrementalBackupFetchingMetadata,
    CurrentIncrementalBackupUploading
} from "@/lib/model";
import { client } from "../api";
import { components } from "../api/spec";

export function convertCurrentBackupJob(data: components["schemas"]["JobStates"]["backup"]): CurrentBackupJob | null {
    if (data === undefined) {
        return null
    }

    const stage: CurrentIncrementalBackupUploading | CurrentIncrementalBackupFetchingMetadata = (() => {
        if (data.stage.stage === "FetchingMetadata") {
            return {
                tag: "FetchingMetadata"
            }
        } else {
            const uploading = data.stage
            return {
                tag: "Uploading",
                timestamp: new Date(uploading.timestamp),
                parent: uploading.parent ?? null,
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
        startedAt: new Date(data.started_at),
        incremental: {
            stage
        }
    }
}

export function currentJobsQuery() {
    return queryOptions({
        queryKey: ['currentJobs'],
        queryFn: async () => {
            const resp = await client.GET("/jobs")
            if (!resp.response) {
                throw resp.error
            }
            return resp.data?.backup
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
    return {
        ...query,
        data: null
    }
}
