use fastx::cli::*;
use fastx::grep::*;
use fastx::stats::*;

fn main() {
    let cli = parse_args();

    match &cli.command {
        Commands::Stats(args) => run_stats(args),
        Commands::Grep(args) => run_grep(args),
    }
}
