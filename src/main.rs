mod subcommands;
mod math_args;

use clap::{Parser, Subcommand, ValueEnum};
use crate::math_args::MathArgs;


fn main() {
    let m_args = MathArgs::parse();

    use crate::subcommands::MathCommands::*;
    match m_args.command {

        Some(Convert { conversion, amount, list } ) => {

            use crate::subcommands::convert::conversions::Conversions::*;
            match conversion {
                FeetToMeters => {
                    println!("{initial:.2} ft -> {result:.2} m", initial = amount, result = amount * 0.3048);
                },
                MetersToFeet => {
                    println!("{initial:.2} ft -> {result:.2} m", initial = amount, result = amount / 0.3048);
                },
                RadiansToDegrees => {
                    println!("{initial:.2} rad -> {result:.2} \u{00B0}", initial = amount, result = amount * (180f64 / std::f64::consts::PI));
                },
                DegreesToRadians => {
                    println!("{initial:.2} \u{00B0} -> {result:.2} rad", initial = amount, result = amount * (std::f64::consts::PI / 180f64))
                },
                _ => {}
            }
        },
        Some(Geometry { list }) => {
            if list {
                use crate::subcommands::geometry::formulas::*;

                for formula in Formulas::value_variants() {
                    println!("{}", formula);
                }
            }
        }

        _ => {

        }
    }
}
