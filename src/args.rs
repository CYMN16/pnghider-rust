use crate::commands::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(name = "pnghider")]
#[command(about = "Hides/reads hidden messages in pngs", version, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}
