use std::fmt::{Display, Formatter};
use std::string::ToString;
use clap::ValueEnum;
use crate::subcommands::geometry::formulas::Formulas::*;

// const PI: &str = "\u{03C0}";
// const POW_2: &str = "\u{00B2}";
// const POW_3: &str = "\u{00B3}";
// const SUBSCRIPT_1: &str = "\u{2082}";
const CIRCLE_AREA: &str = "\u{03C0}r\u{00B2}";
const CIRCLE_CIRCUMFERENCE: &str = "2\u{03C0}r";
const CIRCLE_VOLUME: &str = "4/3\u{03C0}r\u{00B3}";
const TRIANGLE_AREA: &str = "1/2bh";
const TRAPEZOID_AREA: &str = "1/2(b\u{2081}+b\u{2082})h";
const CONE_VOLUME: &str = "1/3\u{03C0}r\u{00B2}h";
const CYLINDER_VOLUME: &str = "\u{03C0}r\u{00B2}h";
const CUBE_VOLUME: &str = "s\u{00B3}";


#[derive(ValueEnum, Clone)]
pub enum Formulas {

    CircleArea,
    CircleCircumference,
    CircleVolume,
    TriangleArea,
    TrapezoidArea,
    ConeVolume,
    CylinderVolume,
    CubeVolume
}
impl Display for Formulas {
    // I need to use a macro here and pass in the formatter as the argument.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<25} {}\n", &self.to_possible_value().unwrap().get_name(),
               match &self {
                CircleArea => { CIRCLE_AREA.to_string() }
                CircleCircumference => { CIRCLE_CIRCUMFERENCE.to_string() }
                CircleVolume => { CIRCLE_VOLUME.to_string() }
                TriangleArea => { TRIANGLE_AREA.to_string() }
                TrapezoidArea => { TRAPEZOID_AREA.to_string() }
                ConeVolume => { CONE_VOLUME.to_string() }
                CylinderVolume => { CYLINDER_VOLUME.to_string() }
                CubeVolume => { CUBE_VOLUME.to_string() }

            }


        )
    }
}