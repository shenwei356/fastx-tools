use fastx_tools::cli::*;
use fastx_tools::grep::*;
use fastx_tools::seq::*;
use fastx_tools::seq_needletail::*;
use fastx_tools::stats::*;

#[cfg(feature = "pprof")]
use pprof::protos::Message;
#[cfg(feature = "pprof")]
use scopeguard::defer;
#[cfg(feature = "pprof")]
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let cli = parse_args();

    fn run_command(cli: &Cli) -> anyhow::Result<()> {
        match &cli.command {
            Commands::Seq(args) => run_seq(args, cli),
            Commands::SeqNeedletail(args) => run_seq_needletail(args, cli),
            Commands::Stats(args) => run_stats(args, cli),
            Commands::Grep(args) => run_grep(args, cli),
        }
    }

    #[cfg(feature = "pprof")]
    {
        if cli.pprof {
            let guard = pprof::ProfilerGuardBuilder::default()
                .frequency(1000)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap();

            defer! {
                match guard.report().build() {
                    Ok(report) => {
                        let mut file = std::fs::File::create("profile.pb").unwrap();
                        let profile = report.pprof().unwrap();

                        let mut content = Vec::new();
                        profile.encode(&mut content).unwrap();
                        file.write_all(&content).unwrap();
                    }
                    Err(_) => {},
                };
            }

            run_command(&cli)
        } else {
            run_command(&cli)
        }
    }

    #[cfg(not(feature = "pprof"))]
    {
        run_command(&cli)
    }
}
