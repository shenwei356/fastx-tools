use fastx_tools::cli::*;
use fastx_tools::grep::*;
use fastx_tools::seq::*;
use fastx_tools::seq_needletail::*;
use fastx_tools::stats::*;

fn main() -> anyhow::Result<()> {
    let cli = parse_args();

    match &cli.command {
        Commands::Seq(args) => run_seq(args),
        Commands::SeqNeedletail(args) => run_seq_needletail(args),

        Commands::Stats(args) => run_stats(args),
        Commands::Grep(args) => run_grep(args),
    }
}
