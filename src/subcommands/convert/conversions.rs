use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Conversions {
    MetersToFeet,
    FeetToMeters,
    RadiansToDegrees,
    DegreesToRadians,
}
