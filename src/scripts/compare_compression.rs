use crate::services::data_tunnel::{
    DataTunnel, EncodingDataTunnel, MappedDataTunnel, PassThroughDataTunnel,
};
use std::fs::File;
use std::io::{Read, Sink, Write};

#[derive(Debug)]
enum CompressionAlgos {
    None,
    Gzip { level: u32 },
    Zstd { level: u32 },
    Brotli { level: u32 },
}

impl CompressionAlgos {
    fn into_tunnel(self) -> MappedDataTunnel<Box<dyn Read>, Box<dyn Write>> {
        match self {
            CompressionAlgos::None => MappedDataTunnel::new(|reader| reader, |writer| writer),
            CompressionAlgos::Gzip { level } => MappedDataTunnel::new(
                |reader| reader,
                move |writer| {
                    Box::new(flate2::write::GzEncoder::new(
                        writer,
                        flate2::Compression::new(level),
                    )) as Box<dyn Write>
                },
            ),
            CompressionAlgos::Zstd { level } => MappedDataTunnel::new(
                |reader| reader,
                move |writer| {
                    Box::new(zstd::stream::write::Encoder::new(writer, level as i32).unwrap())
                        as Box<dyn Write>
                },
            ),
            CompressionAlgos::Brotli { level } => MappedDataTunnel::new(
                |reader| reader,
                move |writer| {
                    Box::new(brotli2::write::BrotliEncoder::new(writer, level)) as Box<dyn Write>
                },
            ),
        }
    }
}

#[test]
fn compare_compression() {
    // Zstd:9 best
    let algos = vec![
        CompressionAlgos::None,
        CompressionAlgos::Gzip { level: 1 },
        CompressionAlgos::Gzip { level: 5 },
        //CompressionAlgos::Gzip { level: 9},
        CompressionAlgos::Zstd { level: 1 },
        CompressionAlgos::Zstd { level: 3 },
        CompressionAlgos::Zstd { level: 5 },
        CompressionAlgos::Zstd { level: 9 },
        //CompressionAlgos::Zstd { level: 15},
        //CompressionAlgos::Zstd { level: 22},
        CompressionAlgos::Brotli { level: 1 },
        CompressionAlgos::Brotli { level: 5 },
        //CompressionAlgos::Brotli { level: 9 }
    ];

    for algo in algos {
        run_compression_algo(algo);
    }
}

fn run_compression_algo(algo: CompressionAlgos) {
    let file = File::open("resources/testing/samples/btrfs_send_wo_parent.bin").unwrap();
    println!("{:?}", format!("{:?}", &algo).replace("\n", ""));
    run_data_tunnel(file, algo.into_tunnel());
}

fn run_data_tunnel(file: File, tunnel: impl DataTunnel) {
    let sink = std::io::sink();

    let transfer = tunnel.tracked_transfer(file, sink);

    let start_time = std::time::Instant::now();
    transfer.run().unwrap();
    let end_time = std::time::Instant::now();

    let read_bytes = transfer.reader_bytes_count();
    let written_bytes = transfer.writer_bytes_count();

    let duration = (end_time - start_time).as_secs_f64();
    println!(
        "Compression ratio: {:.2}%",
        (written_bytes as f64 / read_bytes as f64) * 100.0
    );
    println!(
        "Speed: {:.2}MB/s",
        read_bytes as f64 / 1024.0 / 1024.0 / duration
    );
    println!()
}
