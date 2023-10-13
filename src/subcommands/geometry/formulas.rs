const PI: &str = "\u{03C0}";
const POW_2: &str = "\u{00B2}";
const POW_3: &str = "\u{00B3}";
const CIRCLE_AREA: &str = "\u{03C0}r\u{00B2}";
const CIRCLE_CIRCUMFERENCE: &str = "2\u{03C0}r";
const CIRCLE_VOLUME: &str = "4/3\u{03C0}r\u{00B3}";


pub fn get_all_geometry_formulas() -> Vec<(String, String)> {
    return vec![
        ("circle area".to_string() ,CIRCLE_AREA.to_string()),
        ("circle circumference".to_string(), CIRCLE_CIRCUMFERENCE.to_string()),
        ("circle volume".to_string(), CIRCLE_VOLUME.to_string()),



    ]
}