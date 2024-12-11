use clap::arg;
use clap::value_parser;
use clap::{command, Command};
use clap::{Parser, Subcommand};

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    // #[arg(action = clap::ArgAction::Count)]
    #[arg(long, action = clap::ArgAction::Count)]
    pub img_resize: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    pub word_count: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    pub parallel: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    pub sequential: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    pub javakv_test: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    pub with_ow: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    pub with_wl: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    pub bench_mode: u8,

    // create many function copy and collect the average cold start
    #[arg(long, action = clap::ArgAction::Count)]
    pub first_call_mode: u8,
}
