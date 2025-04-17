import {CurrentBackupJob} from "@/lib/model";
import {formatDateTime} from "@/lib/format/dates";
import {Spinner, Switch} from "@heroui/react";
import {Chip} from "@heroui/chip";
import {Clock} from "@deemlol/next-icons";
import {formatBytesBase10} from "@/lib/format/bytes";

export default function CurrentBackupContent(
    {data}: { data: CurrentBackupJob }
) {
    return (
        <div className="w-full h-full flex flex-col gap-2">
            <Chip variant="faded" startContent={<Clock className="w-full h-full p-0.5"/>} size="sm"
                  className="text-gray-800">
                {formatDateTime(data.startedAt)}
            </Chip>
            {data.incremental.stage.tag === "FetchingMetadata" &&
                <div className="w-full grow text-gray-600 flex justify-center items-center">
                    <Spinner size="lg" color="secondary" label="Fetching Metadata..."/>
                </div>
            }
            {data.incremental.stage.tag === "Uploading" &&
                <div className="w-full grow flex flex-col text-gray-800 text-small">
                    {data.incremental.stage.finishing ? <div className="w-full grow flex justify-center items-center">
                        <Spinner size="lg" color="primary" label="Finishing..."/>
                    </div> : <div className="grid grid-cols-3 grid-rows-3 grid-flow-col gap-1">
                        <div className="flex flex-col">
                            <label className="text-small text-gray-600">Parent</label>
                            <p className="text-medium text-gray-800 font-medium">
                                {data.incremental.stage.parent ? data.incremental.stage.parent :
                                    <span className="text-gray-700">None</span>}
                            </p>
                        </div>

                        <div className="flex flex-col">
                            <label className="text-small text-gray-600">Local Snapshot</label>
                            <p className="text-small text-gray-800 font-medium bg-gray-200 ring-2 ring-gray-300 shadow-sm rounded-md px-1 w-fit">
                                {data.incremental.stage.localSnapshot}
                            </p>
                        </div>

                        <div className="flex flex-col">
                            <label className="text-small text-gray-600">Remote Filename</label>
                            <p className="text-small text-gray-800 font-medium bg-gray-200 ring-2 ring-gray-300 shadow-sm rounded-md px-1 w-fit">
                                {data.incremental.stage.remoteFilename}
                            </p>
                        </div>


                        <div className="flex flex-col">
                            <label className="text-small text-gray-600">Compression</label>
                            <p className="text-medium text-gray-800 font-medium">
                                {data.incremental.stage.compressionLevel}
                                {data.incremental.stage.compressionLevel !== "None" &&
                                    `, ${(data.incremental.stage.bytesWritten / data.incremental.stage.bytesRead * 100).toFixed(2)}%`}
                            </p>
                        </div>

                        <div className="flex flex-col row-span-2">
                            <label className="text-small text-gray-600">Encrypted</label>
                            <p className="text-medium text-gray-800 font-medium">
                                <Switch classNames={{
                                    wrapper: "w-12 h-5",
                                    thumb: [
                                        "h-4 w-4 group-data-[selected=true]:ms-6",
                                    ]
                                }} isReadOnly isSelected={data.incremental.stage.encrypted} size="sm" color="primary"/>
                            </p>
                        </div>


                        <div className="flex flex-col">
                            <label className="text-small text-gray-600">Raw Size</label>
                            <p className="text-medium text-gray-800 font-medium">
                                {formatBytesBase10(data.incremental.stage.bytesRead)}
                            </p>
                        </div>

                        <div className="flex flex-col">
                            <label className="text-small text-gray-600">Speed</label>
                            <p className="text-medium text-gray-800 font-medium">
                                {formatBytesBase10(data.incremental.stage.bytesWrittenPerSecond)}/s
                            </p>
                        </div>

                        <div className="flex flex-col">
                            <label className="text-small text-gray-600">Remote Size</label>
                            <p className="text-medium text-gray-800 font-medium">
                                {formatBytesBase10(data.incremental.stage.bytesWritten)}
                            </p>
                        </div>
                    </div>}
                </div>}
        </div>
    );
}