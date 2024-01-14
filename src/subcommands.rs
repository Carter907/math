use crate::subcommands::calculus::laws::Laws;
use crate::subcommands::convert::conversions::ConversionType;
use clap::Subcommand;

pub mod calculus;
pub mod convert;
pub mod formula;
pub mod geometry;
pub mod prefix;

#[derive(Subcommand, Debug)]
pub enum MathCommands {
    Convert {
        #[command(subcommand)]
        conversion_type: ConversionType,
        #[arg(short, long)]
        amount: f64,
        #[arg(short, long)]
        list: bool,
    },
    Geometry {
        #[arg(short, long)]
        list: bool,
    },
    Calculus {
        #[command(subcommand)]
        laws: Option<Laws>,
    },
    Prefix {
        #[arg(short, long)]
        test: bool,
        #[arg(short, long)]
        cheat_sheet: bool,
    },
}
