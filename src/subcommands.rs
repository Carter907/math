use clap::Subcommand;
use crate::subcommands::convert::conversions::Conversions;

pub mod geometry;
pub mod convert;

#[derive(Subcommand, Debug)]
pub enum MathCommands {
    Convert {
        #[command(subcommand)]
        conversion: Conversions,
        #[arg(short, long)]
        amount: f64,
        #[arg(short,long)]
        list: bool

    },
    Geometry {
        #[arg(short, long)]
        list: bool
    }
}
