use crate::cli::*;
use fastx;

pub fn run_stats(args: &StatsArgs) {
    let mut writer = fastx::util::xwrite(&args.out_file).unwrap_or_else(|e| {
        eprintln!("failed to write to file {}: {}", &args.out_file, e);
        std::process::exit(1);
    });

    for file in &args.files {
        match fastx::Reader::new(&file) {
            Ok(mut reader) => {
                while let Some(seq) = reader.next() {
                    if seq.is_fastq() {
                        let _ = writeln!(
                            writer,
                            "@{}\n{}\n+\n{}",
                            String::from_utf8_lossy(seq.id),
                            String::from_utf8_lossy(seq.seq),
                            String::from_utf8_lossy(seq.qual.unwrap()),
                        );
                    } else {
                        let _ = writeln!(
                            writer,
                            ">{}\n{}",
                            String::from_utf8_lossy(seq.id),
                            String::from_utf8_lossy(seq.seq),
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!("failed to read input file {}: {}", file, e)
            }
        }
    }
}
