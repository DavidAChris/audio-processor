mod validate;

use clap::{command, Parser};
use validate::{check_accuracy_value, check_wav_file};

const DEFAULT_LOG_FILE: &str = "WavSilenceRemoval.log";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    /// Wav file to analyze
    #[arg(short, long, value_parser = check_wav_file)]
    pub(crate) file: String,

    /// Accuracy Level (1/nth of a second): [1, 2, 4, 6, 10, 30, 80, 100]
    #[arg(short, long, value_parser = check_accuracy_value, default_value_t = 2)]
    pub(crate) accuracy: i32,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) debug: u8,

    /// Run the utility to view the results only
    #[arg(long, action = clap::ArgAction::Count)]
    pub(crate) dry_run: u8,

    /// Change the default log file
    #[arg(short, long, default_value_t = String::from(DEFAULT_LOG_FILE))]
    pub(crate) log_file: String,

    /// Disable file logging
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) no_log_file: u8,
}
