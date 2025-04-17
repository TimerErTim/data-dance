import {queryOptions, useQuery} from "@tanstack/react-query";
import config from "@/lib/config";
import {JobHistoryAPI} from "@/lib/queries/spec";
import {HistoryBackupJob} from "@/lib/model";


function convertHistoryBackupJob(entry: JobHistoryAPI['entries'][0]): HistoryBackupJob | null {
    if (!!entry.Restore) {
        return null
    }

    if (!!entry.IncrementalBackup.state.Error) {
        return {
            startedAt: new Date(entry.IncrementalBackup.started_at),
            finishedAt: new Date(entry.IncrementalBackup.finished_at),
            result: {
                _type: "Error",
                value: entry.IncrementalBackup.state.Error
            }
        }
    }

    return {
        startedAt: new Date(entry.IncrementalBackup.started_at),
        finishedAt: new Date(entry.IncrementalBackup.finished_at),
        result: {
            _type: "Success",
            id: entry.IncrementalBackup.state.Success.id,
            parent: entry.IncrementalBackup.state.Success.parent,
            remoteFilename: entry.IncrementalBackup.state.Success.remote_filename,
            localSnapshot: entry.IncrementalBackup.state.Success.local_snapshot,
            bytesRead: entry.IncrementalBackup.state.Success.bytes_read,
            bytesWritten: entry.IncrementalBackup.state.Success.bytes_written,
            compressionLevel: entry.IncrementalBackup.state.Success.compression_level,
            encrypted: entry.IncrementalBackup.state.Success.encrypted
        }
    }
}


export function historyJobsQuery() {
    return queryOptions({
        queryKey: ['historyJobs'],
        queryFn: async () => {
            const response = await fetch(config.host + '/api/jobs/history');
            const payload = await response.json();
            return payload as JobHistoryAPI
        },
        refetchInterval: 2500
    })
}

export function useHistoryBackupJobs() {
    const query = useQuery(historyJobsQuery())

    if (!query.data) {
        return query
    }

    const newData: HistoryBackupJob[] = []

    for (const entry of query.data.entries) {
        const convertedEntry = convertHistoryBackupJob(entry)
        if (!convertedEntry) {
            continue
        }
        newData.push(convertedEntry)
    }

    return {
        ...query,
        data: {
            entries: newData
        }
    }
}
