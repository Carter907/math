use clap::Subcommand;
use crate::subcommands::calculus::laws::Laws;
use crate::subcommands::convert::conversions::Conversions;

pub mod geometry;
pub mod convert;
pub mod calculus;
pub mod formula;

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
    },
    Calculus {
        #[command(subcommand)]
        laws: Option<Laws>
    }

}
