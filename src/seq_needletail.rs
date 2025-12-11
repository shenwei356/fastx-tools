use crate::cli::*;
use anyhow::Context;
use fastx::util::xwrite;

use needletail::parse_fastx_file;

pub fn run_seq_needletail(args: &SeqNeedletailArgs) -> anyhow::Result<()> {
    let mut writer = xwrite(&args.out_file, 65536)
        .with_context(|| format!("failed to write to file: {}", &args.out_file))?;

    for file in &args.files {
        let mut reader = parse_fastx_file(&file)
            .with_context(|| format!("failed to parse input file: {}", file))?;

        while let Some(result) = reader.next() {
            let record = result?;

            match record.qual() {
                Some(qual) => {
                    let _ = writeln!(
                        writer,
                        "@{}\n{}\n+\n{}",
                        String::from_utf8_lossy(record.id()),
                        String::from_utf8_lossy(&record.seq()),
                        String::from_utf8_lossy(qual),
                    );
                }
                _ => {
                    let _ = writeln!(
                        writer,
                        ">{}\n{}",
                        String::from_utf8_lossy(record.id()),
                        String::from_utf8_lossy(&record.seq()),
                    );
                }
            }
        }
    }
    Ok(())
}
