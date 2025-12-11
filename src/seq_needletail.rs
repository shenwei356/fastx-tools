use crate::cli::*;
use anyhow::Context;
use fastx::util::xwrite;

use needletail::parse_fastx_file;

pub fn run_seq_needletail(args: &SeqNeedletailArgs, global: &Cli) -> anyhow::Result<()> {
    let mut writer = xwrite(&global.out_file, 65536)
        .with_context(|| format!("failed to write to file: {}", &args.out_file))?;

    let fa_mark: &[u8] = b">";
    let fq_mark: &[u8] = b"@";
    let fq_sep: &[u8] = b"\n+\n";
    let lf: &[u8] = b"\n";

    for file in &args.files {
        let mut reader = parse_fastx_file(&file)
            .with_context(|| format!("failed to parse input file: {}", file))?;

        while let Some(result) = reader.next() {
            let seq = result?;

            match seq.qual() {
                Some(qual) => {
                    writer.write_all(fq_mark)?;
                    writer.write_all(seq.id())?;
                    writer.write_all(lf)?;
                    writer.write_all(&seq.seq())?;
                    writer.write_all(fq_sep)?;
                    writer.write_all(qual)?;
                    writer.write_all(lf)?;
                }
                _ => {
                    writer.write_all(fa_mark)?;
                    writer.write_all(seq.id())?;
                    writer.write_all(lf)?;
                    writer.write_all(&seq.seq())?;
                    writer.write_all(lf)?;
                }
            }
        }
    }
    Ok(())
}
