import {HistoryBackupJob} from "@/lib/model";
import {Accordion, AccordionItem, Button, DateRangePicker, RangeValue} from "@heroui/react";
import {formatDateTime} from "@/lib/format/dates";
import {formatBytesBase10} from "@/lib/format/bytes";
import {Clock, Minus, Upload, XCircle} from "@deemlol/next-icons";
import {Chip} from "@heroui/react";
import {useState} from "react";
import {CalendarDate} from "@internationalized/date";

export default function HistoryBackupContent(
    {historyBackups}: { historyBackups: HistoryBackupJob[] }
) {
    const [dateRange, setDateRange] = useState<RangeValue<CalendarDate> | null>(null);

    function HistoryEntry({backup}: { backup: HistoryBackupJob }) {
        if (backup.result._type == "Error") {
            return <div className="flex flex-col gap-1">
                <Chip variant="faded" size="sm"
                      className="text-gray-800">
                    <div className="flex flex-row gap-1 items-center">
                        {formatDateTime(backup.startedAt)}
                        <Minus className="w-full h-full p-0.5"/>
                        {formatDateTime(backup.finishedAt)}
                    </div>
                </Chip>
                <p className="p-2 text-xl text-danger">{backup.result.value}</p>
            </div>
        }

        return (<div className="flex flex-col gap-1">
            <div className="flex flex-row justify-between flex-wrap">
                <Chip variant="faded" size="sm"
                      className="text-gray-800">
                    <div className="flex flex-row gap-1 items-center">
                        {formatDateTime(backup.startedAt)}
                        <Minus className="w-full h-full p-0.5"/>
                        {formatDateTime(backup.finishedAt)}
                    </div>
                </Chip>
                <Button color="primary" size="sm" className="px-6 py-0 leading-none">
                    Restore
                </Button>
            </div>
            <div className="flex flex-row gap-2">
                <p>{formatDateTime(backup.startedAt)}</p>
                <p>{formatBytesBase10(backup.result.bytesWritten)}</p>
            </div>
        </div>);
    }

    const filteredHistoryBackups = dateRange ? historyBackups.filter((backup) => {
        const backupDate = new CalendarDate(backup.startedAt.getFullYear(), backup.startedAt.getMonth() + 1, backup.startedAt.getDate());
        return backupDate >= dateRange.start && backupDate <= dateRange.end;
    }) : historyBackups;

    const sortedFilteredHistoryBackups = filteredHistoryBackups.sort((a, b) => b.startedAt.getTime() - a.startedAt.getTime());

    return (
        <div className="flex flex-col gap-2 grow flex-1 min-h-0">
            <DateRangePicker variant="bordered" visibleMonths={2} color="primary" labelPlacement="outside-left"
                             label="Filter by date" value={dateRange} onChange={setDateRange}/>
            <Accordion isCompact={true} selectionBehavior="toggle" selectionMode="multiple" showDivider={false}
                       className="flex flex-col gap-1 flex-1 min-h-0 overflow-y-auto bg-transparent px-0" itemClasses={{
                base: "py-0 px-0 w-full shadow-lg border rounded-lg bg-white",
                title: "font-normal text-medium flex flex-row items-center",
                trigger: "p-2 data-[hover=true]:bg-default-100 rounded-lg flex items-center",
                content: "p-2 text-small",
            }}>
                {sortedFilteredHistoryBackups.map((backup) => {
                    return <AccordionItem key={backup.startedAt.getTime()} title={<div className="flex flex-row gap-2">
                        <Chip variant="faded" startContent={<Clock className="w-full h-full p-0.5"/>} size="sm"
                              className="text-gray-800">
                            {formatDateTime(backup.startedAt)}
                        </Chip>
                        <div className="text-medium text-gray-800 font-medium flex flex-row gap-1">
                            {backup.result._type == "Error" ? <XCircle className="text-danger"/> : <>
                                <Upload/>
                                {formatBytesBase10(backup.result.bytesWritten)}
                            </>}
                        </div>
                    </div>}>
                        <HistoryEntry backup={backup}/>
                    </AccordionItem>
                })}
            </Accordion>
        </div>
    );
}