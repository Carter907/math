mod math_args;
mod subcommands;

use crate::math_args::MathArgs;
use clap::{Parser, Subcommand, ValueEnum};
use crate::subcommands::convert::Conversion;

fn main() {
    let m_args = MathArgs::parse();

    use crate::subcommands::MathCommands::*;
    match m_args.command {
        Some(Convert {
            conversion_type,
            amount,
            list,
        }) => {
            use crate::subcommands::convert::conversions::ConversionType::*;
            
            let conversion = Conversion { conversion_type: conversion_type, initial: 0, result: 0 }
            
           
        }
        Some(Geometry { list }) => {
            if list {
                use crate::subcommands::geometry::formulas::*;

                for formula in Formulas::value_variants() {
                    println!("{}", formula);
                }
            }
        }
        Some(Prefix { cheat_sheet, test }) => {
            
        }
        _ => {}
    }
}
