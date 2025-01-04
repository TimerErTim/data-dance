use crate::objects::job_state::{
    BackupJobState, IncrementalBackupStage, IncrementalBackupUploadState, JobStates,
};
use humansize::{make_format, FormatSizeOptions};
use leptos::logging;
use leptos::{
    component, create_effect, create_memo, create_owning_memo, create_signal, view, IntoView,
    ReadSignal, Show, Signal,
};

#[component]
pub fn RunningBackupStats(
    #[prop(into)] upload_stats: Signal<IncrementalBackupUploadState>,
) -> impl IntoView {
    let bytes_formatter = move || {
        make_format(
            FormatSizeOptions::default()
                .decimal_zeroes(1)
                .decimal_places(2)
                .space_after_value(true),
        )
    };

    let parent_backup = create_memo(move |_| upload_stats().parent);
    let finishing = create_memo(move |_| upload_stats().finishing);

    let (write_speed, set_write_speed) = create_signal(0);
    create_effect(move |prev| {
        let upload = upload_stats();
        if let Some((prev_bytes, prev_time)) = prev {
            let byte_difference = upload.bytes_written - prev_bytes;
            let time_difference = upload.timestamp.signed_duration_since(prev_time);
            if time_difference.num_milliseconds() > 0 {
                let new_write_speed = (byte_difference as f64
                    / (time_difference.num_milliseconds() as f64 / 1000.0))
                    as u64;
                set_write_speed(new_write_speed);
            }
        }
        return (upload.bytes_written, upload.timestamp);
    });
    let bytes_written = create_memo(move |_| upload_stats().bytes_written);
    let bytes_read = create_memo(move |_| upload_stats().bytes_read);
    let compression_ratio = create_memo(move |_| {
        if bytes_written() > 0 {
            let ratio = bytes_written() as f64 / bytes_read() as f64;
            return ratio;
        }
        return 1.0;
    });
    let encrypted = create_memo(move |_| upload_stats().encrypted);
    let compression_level = create_memo(move |_| upload_stats().compression_level);

    view! {
        <div class="flex flex-col items-center p-4 text-gray-700 text-sm">
            {move || match parent_backup() {
                Some(parent) => view! {
                    <p>"Parent: "
                        <span class="font-medium text-gray-800 text-md">
                            {parent}
                        </span>
                    </p>
                },
                None => view! {
                    <p class="font-medium text-md">"No parent"</p>
                },
            }}
            <p>"Uploaded: "
                <span class="font-medium text-gray-800 text-md">
                    {move || format!("{}", bytes_formatter()(bytes_written()))}
                </span>
            </p>
            <p>"Compression ratio: "
                <span class="font-medium text-gray-800 text-md">
                    {move || format!("{:.2}%", compression_ratio() * 100.0)}
                </span>
            </p>
            <p>"Encrypted: "
                <span class="font-medium text-gray-800 text-md">
                    {move || format!("{}", encrypted())}
                </span>
            </p>
            <p>"Compression level: "
                <span class="font-medium text-gray-800 text-md">
                    {move || format!("{:?}", compression_level())}
                </span>
            </p>
            <Show when=move || !finishing() fallback=move || view! {
                <p>"Finishing..."</p>
            }>
                <p>"Write speed: "
                    <span class="font-medium text-gray-800 text-md">
                        {move || format!("{}/s", bytes_formatter()(write_speed()))}
                    </span>
                </p>
            </Show>
        </div>

    }
}
