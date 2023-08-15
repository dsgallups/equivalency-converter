#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitType {
    Length,
    Mass,
    Time,
    ElectricCurrent,
    Temperature,
    Amount,
    LuminousIntensity,
}

#[derive(Debug, PartialEq)]
pub struct Unit {
    pub r#type: UnitType,
    pub name: &'static str,
    pub symbol: &'static str,
    pub measure: &'static str,
}

use UnitType::*;

type Power = i32;
struct DerivedMeasurement<'a> {
    symbols: Vec<&'static str>,
    units: Vec<(&'a Unit, Power)>,
}

fn main() {
    let length = Unit {
        r#type: Length,
        name: "meter",
        symbol: "m",
        measure: "length",
    };
    let mass = Unit {
        r#type: Mass,
        name: "kilogram",
        symbol: "kg",
        measure: "mass",
    };
    let time = Unit {
        r#type: Time,
        name: "second",
        symbol: "s",
        measure: "time",
    };
    let current = Unit {
        r#type: ElectricCurrent,
        name: "ampere",
        symbol: "A",
        measure: "electric current",
    };

    let temp = Unit {
        r#type: Temperature,
        name: "kelvin",
        symbol: "K",
        measure: "thermodynamic temperature",
    };

    let amount = Unit {
        r#type: Amount,
        name: "mole",
        symbol: "mol",
        measure: "amount of substance",
    };

    let lum = Unit {
        r#type: LuminousIntensity,
        name: "candela",
        symbol: "cd",
        measure: "luminous intensity",
    };

    let derived_measurements: Vec<DerivedMeasurement> = vec![
        DerivedMeasurement {
            symbols: vec!["speed-of-light"],
            units: vec![(&mass, 1), (&time, -1)],
        },
        DerivedMeasurement {
            symbols: vec!["gravitational-constant"],
            units: vec![(&length, 3), (&mass, -1), (&time, -2)],
        },
        DerivedMeasurement {
            symbols: vec!["energy"],
            units: vec![(&mass, 1), (&length, 2), (&time, -2)],
        },
        DerivedMeasurement {
            symbols: vec!["power", "watt"],
            units: vec![(&mass, 1), (&length, 2), (&time, -3)],
        },
    ];

    /*
       DerivedMeasurement_name: "c",
       units: vec![{m, 1}, {s, -1}]
    */

    println!("Hello, world!");
}
