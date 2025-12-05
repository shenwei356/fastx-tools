use fastx_tools::cli::*;
use fastx_tools::grep::*;
use fastx_tools::stats::*;

fn main() {
    let cli = parse_args();

    match &cli.command {
        Commands::Stats(args) => run_stats(args),
        Commands::Grep(args) => run_grep(args),
    }
}
