use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct MathArgs {

    #[command(subcommand)]
    command: Option<MathCommands>

}
#[derive(Subcommand, Debug)]
enum MathCommands {
    Convert {
        #[command(subcommand)]
        conversion: Conversions,
        #[arg(short, long)]
        amount: f64,
    }
}
#[derive(Subcommand, Debug)]
enum Conversions {
    MetersToFeet,
    FeetToMeters,
    RadiansToDegrees,
    DegreesToRadians,
}

fn main() {
    let m_args = MathArgs::parse();

    use crate::MathCommands::Convert;
    match m_args.command {

        Some(Convert { conversion, amount } ) => {

            use crate::Conversions::*;
            match conversion {
                FeetToMeters => {
                    println!("{initial:.2} ft -> {result:.2} m", initial = amount, result = amount * 0.3048)
                }
                _ => {}
            }

        }
        _ => {

        }
    }
}
