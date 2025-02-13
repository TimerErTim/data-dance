import Image from "next/image";
import {Button, Card, CardBody, CardHeader, Divider} from "@heroui/react";

export default function Home() {
    return (
        <div className="p-2 md:p-4 lg:p-6 h-full">
            <main className="flex flex-row gap-2 md:gap-4 lg:gap-6 h-full">
                <div className="w-full h-full flex flex-col">
                    <Card className="w-full">
                        <CardHeader>
                            <h1>Backups</h1>
                        </CardHeader>
                        <CardBody>
                            <Button>Backup</Button>
                        </CardBody>
                    </Card>

                    <div className="bg-red-500 grow">

                    </div>
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
