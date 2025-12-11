use crate::cli::*;
use anyhow::Context;
use fastx::util::xwrite;

pub fn run_seq(args: &SeqArgs, global: &Cli) -> anyhow::Result<()> {
    let mut writer = xwrite(&global.out_file, 65536)
        .with_context(|| format!("failed to write to file: {}", &global.out_file))?;

    let space: &[u8] = b" ";
    let fa_mark: &[u8] = b">";
    let fq_mark: &[u8] = b"@";
    let fq_sep: &[u8] = b"\n+\n";
    let lf: &[u8] = b"\n";

    for file in &args.files {
        let mut reader = fastx::Reader::new(&file)
            .with_context(|| format!("failed to parse input file: {}", file))?;

        while let Some(res) = reader.next() {
            let seq = res?;

            if seq.is_fastq() {
                writer.write_all(fq_mark)?;
                writer.write_all(seq.id)?;
                if seq.desc.len() > 0 {
                    writer.write_all(space)?;
                    writer.write_all(seq.desc)?;
                }
                writer.write_all(lf)?;
                writer.write_all(seq.seq)?;
                writer.write_all(fq_sep)?;
                writer.write_all(seq.qual.unwrap())?;
                writer.write_all(lf)?;
            } else {
                writer.write_all(fa_mark)?;
                writer.write_all(seq.id)?;
                if seq.desc.len() > 0 {
                    writer.write_all(space)?;
                    writer.write_all(seq.desc)?;
                }
                writer.write_all(lf)?;
                writer.write_all(seq.seq)?;
                writer.write_all(lf)?;
            }
        }
    }

    Ok(())
}
