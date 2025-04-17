"use client"

import {useHistoryBackupJobs} from "@/lib/queries/history";
import HistoryBackupContent from "@/lib/components/backups/HistoryBackupContent";

export default function HistoryBackupSection() {
    const historyBackupsQuery = useHistoryBackupJobs()

    return (
        <section className="flex flex-col gap-2 flex-1 min-h-0">
            {historyBackupsQuery.data && historyBackupsQuery.data.entries.length > 0 ? (
                <HistoryBackupContent historyBackups={historyBackupsQuery.data?.entries}/>
            ) : <div className="flex h-full w-full items-center justify-center">
                <p className="text-2xl text-gray-600 font-medium">
                    No backups found
                </p>
            </div>}
        </section>
    );
}