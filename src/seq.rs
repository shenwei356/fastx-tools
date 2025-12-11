use crate::cli::*;
use anyhow::Context;
use fastx::util::xwrite;

pub fn run_seq(args: &SeqArgs) -> anyhow::Result<()> {
    let mut writer = xwrite(&args.out_file, 65536)
        .with_context(|| format!("failed to write to file {}", &args.out_file))?;

    for file in &args.files {
        let mut reader = fastx::Reader::new(&file)
            .with_context(|| format!("failed to parse input file {}", file))?;

        while let Some(res) = reader.next() {
            let seq = res?;

            let desc = if seq.desc.len() == 0 {
                "".to_string()
            } else {
                format!(" {}", String::from_utf8_lossy(seq.desc))
            };

            if seq.is_fastq() {
                let _ = writeln!(
                    writer,
                    "@{}{}\n{}\n+\n{}",
                    String::from_utf8_lossy(seq.id),
                    desc,
                    String::from_utf8_lossy(seq.seq),
                    String::from_utf8_lossy(seq.qual.unwrap()),
                );
            } else {
                let _ = writeln!(
                    writer,
                    ">{}{}\n{}",
                    String::from_utf8_lossy(seq.id),
                    desc,
                    String::from_utf8_lossy(seq.seq),
                );
            }
        }
    }
    Ok(())
}
