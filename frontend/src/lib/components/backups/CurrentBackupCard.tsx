"use client"

import {Button, Card, CardBody, CardHeader, Divider} from "@heroui/react";
import {useCurrentBackupJob} from "@/lib/queries/jobs";
import {useState} from "react";
import {useStartBackupMutation} from "@/lib/queries/backup";
import CurrentBackupContent from "@/lib/components/backups/CurrentBackupContent";
import {CloudOff, UploadCloud} from "@deemlol/next-icons";

export default function CurrentBackupCard() {
    const [startButtonDisabled, setStartButtonDisabled] = useState(false)
    const currentBackupJob = useCurrentBackupJob()
    const startBackupJob = useStartBackupMutation({
        onSuccess: () => {
            setStartButtonDisabled(false)
        },
        onError: () => {
            setStartButtonDisabled(false)
        }
    })

    const isStartButtonDisabled = !!currentBackupJob.data || startButtonDisabled
    return (
        <Card fullWidth>
            <CardHeader className="flex flex-row items-center justify-between p-2">
                <div className="flex flex-row gap-4 text-gray-800 items-center">
                    <h1 className="font-medium text-lg">Backups</h1>
                    <UploadCloud/>
                </div>
                <Button color="primary" size="sm" onPress={() => {
                    setStartButtonDisabled(true)
                    startBackupJob.mutate()
                }}
                        isDisabled={isStartButtonDisabled}
                        isLoading={startBackupJob.isPending}
                >
                    Start Backup
                </Button>
            </CardHeader>
            <Divider/>
            <CardBody className="h-48 p-2">
                {!currentBackupJob.data ? (
                    <div className="flex flex-col gap-4 items-center justify-center w-full h-full text-gray-600">
                        <CloudOff className="w-full grow mt-4"/>
                        <p className="text-xl mb-6">No Backup In Progress</p>
                    </div>
                ) : <CurrentBackupContent data={currentBackupJob.data}/>}
            </CardBody>
        </Card>
    );
}