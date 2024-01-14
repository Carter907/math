use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ConversionType {
    MetersToFeet,
    FeetToMeters,
    RadiansToDegrees,
    DegreesToRadians,
    InchesToMeters,
    MilesToKilometers,
    GallonsToLiters,
    CubicFeetToGallons,
}


