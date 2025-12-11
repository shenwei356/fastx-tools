use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    author, 
    version,
    about,
    long_about = None, 
    disable_help_subcommand = true,
    // subcommand_required = false,
    // arg_required_else_help = false,
    // https://docs.rs/clap/latest/clap/struct.Command.html#method.help_template
    help_template = "\
{before-help}{about}

Version: {name} v{version}
 Author: {author}
   Home: https://github.com/shenwei356/fastx

{usage-heading} {usage}

{all-args}{after-help}",
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Simple statistics of FASTA/Q files
    Seq(SeqArgs),

    /// Simple statistics of FASTA/Q files
    SeqNeedletail(SeqNeedletailArgs),

    /// Simple statistics of FASTA/Q files
    Stats(StatsArgs),

    /// Search FASTA/Q records
    Grep(GrepArgs),
}

#[derive(Args, Debug)]
pub struct SeqArgs {
    #[arg(num_args = 1.., default_value = "-", help = "FASTA/Q file(s)")]
    pub files: Vec<String>,

    #[arg(short = 'o', long = "out-file" , default_value = "-", help = "output file")]
    pub out_file: String,
}


#[derive(Args, Debug)]
pub struct SeqNeedletailArgs {
    #[arg(num_args = 1.., default_value = "-", help = "FASTA/Q file(s)")]
    pub files: Vec<String>,

    #[arg(short = 'o', long = "out-file" , default_value = "-", help = "output file")]
    pub out_file: String,
}

#[derive(Args, Debug)]
pub struct StatsArgs {
    #[arg(num_args = 1.., default_value = "-", help = "FASTA/Q file(s)")]
    pub files: Vec<String>,

    #[arg(short = 'o', long = "out-file" , default_value = "-", help = "output file")]
    pub out_file: String,

    #[arg(short = 'a', long = "all", help = "Show all information")]
    pub all: bool,
}

#[derive(Args, Debug)]
pub struct GrepArgs {
    #[arg(num_args = 1.., default_value = "-", help = "FASTA/Q file(s)")]
    pub files: Vec<String>,

    #[arg(short = 'o', long = "out-file" , default_value = "-", help = "output file")]
    pub out_file: String,

    #[arg(
        short = 'p',
        long = "pattern",
        help = "Search pattern to match sequence ID"
    )]
    pub patterns: bool,

    #[arg(
        short = 'r',
        long = "use-regexp",
        help = "Search patterns are regular expression"
    )]
    pub use_regexp: bool,

    #[arg(
        short = 'v',
        long = "invert-match",
        help = "Output non-matching records"
    )]
    pub invert_match: bool,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
