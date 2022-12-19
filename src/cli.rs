use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The callsign to send reports from
    #[clap(short, long)]
    pub callsign: String,

    /// The password to use for the callsign
    #[clap(short, long)]
    pub password: String,

    /// Do a dry run (no publishing)
    #[clap(short, long)]
    pub dry_run: bool,
}