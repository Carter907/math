use std::fmt::{Display, Formatter};
use crate::subcommands::convert::conversions::ConversionType;
use crate::subcommands::convert::conversions::ConversionType::{DegreesToRadians, FeetToMeters, MetersToFeet, RadiansToDegrees};

pub mod conversions;
mod operations;

pub fn print_conversion() {}

pub struct Conversion {
    conversion_type: ConversionType,
    initial: i64,
}

impl Conversion {
    fn convert(&self) -> i64 {
        match &self.conversion_type {
            FeetToMeters => {
                *&self.initial * 0.3048
            }
            MetersToFeet => {
                *&self.initial / 0.3048
            }
            RadiansToDegrees => {
                *&self.initial * (180f64 / std::f64::consts::PI)
            }
            DegreesToRadians => {
                *&self.initial * (std::f64::consts::PI / 180f64)
            }
            _ => { 0 }
        }
        0
    }
}

impl Display for Conversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:.2} -> {:.2}", &self.initial, &self.convert()).as_str())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_conversion() {

    }
}