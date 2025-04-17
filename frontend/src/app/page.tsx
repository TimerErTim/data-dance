import {Card, CardHeader, Divider} from "@heroui/react";
import CurrentBackupCard from "@/lib/components/backups/CurrentBackupCard";
import HistoryBackupSection from "@/lib/components/backups/HistoryBackupSection";

export default function Home() {
    return (
        <div className="p-2 md:p-4 lg:p-6 h-full">
            <main className="flex flex-row gap-2 md:gap-4 lg:gap-6 h-full">
                <div className="w-full h-full flex flex-col gap-4 items-stretch flex-initial min-h-0">
                    <CurrentBackupCard/>

                    <HistoryBackupSection/>
                </div>

                <Divider orientation="vertical" className="h-auto"/>

                <div className="w-full h-full flex flex-col">
                    <Card className="w-full h-full">
                        <CardHeader>
                            <h1>Restoration</h1>
                        </CardHeader>
                    </Card>
                </div>
            </main>
        </div>
    );
}
