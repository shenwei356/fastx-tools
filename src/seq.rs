use crate::cli::*;
use anyhow::Context;
use fastx::util::xwrite;

pub fn run_seq(args: &SeqArgs, global: &Cli) -> anyhow::Result<()> {
    let mut writer = xwrite(&global.out_file, 65536)
        .with_context(|| format!("failed to write to file: {}", &global.out_file))?;

    const SPACE: &[u8] = b" ";
    const FA_MARK: &[u8] = b">";
    const FQ_MARK: &[u8] = b"@";
    const FQ_SEP: &[u8] = b"\n+\n";
    const LF: &[u8] = b"\n";

    for file in &args.files {
        let mut reader = fastx::Reader::new(&file)
            .with_context(|| format!("failed to parse input file: {}", file))?;

        while let Some(res) = reader.next() {
            let seq = res?;

            if seq.is_fastq() {
                writer.write_all(FQ_MARK)?;
                writer.write_all(seq.id)?;
                if seq.desc.len() > 0 {
                    writer.write_all(SPACE)?;
                    writer.write_all(seq.desc)?;
                }
                writer.write_all(LF)?;
                writer.write_all(seq.seq)?;
                writer.write_all(FQ_SEP)?;
                writer.write_all(seq.qual.unwrap())?;
                writer.write_all(LF)?;
            } else {
                writer.write_all(FA_MARK)?;
                writer.write_all(seq.id)?;
                if seq.desc.len() > 0 {
                    writer.write_all(SPACE)?;
                    writer.write_all(seq.desc)?;
                }
                writer.write_all(LF)?;
                writer.write_all(seq.seq)?;
                writer.write_all(LF)?;
            }
        }
    }

    Ok(())
}
