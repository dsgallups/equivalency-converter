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
    name: &'static str,
    symbols: Vec<&'static str>,
    units: Vec<(&'a Unit, Power)>,
}

fn init_base_units() -> (Unit, Unit, Unit, Unit, Unit, Unit, Unit) {
    return (
        Unit {
            r#type: Length,
            name: "meter",
            symbol: "m",
            measure: "length",
        },
        Unit {
            r#type: Mass,
            name: "kilogram",
            symbol: "kg",
            measure: "mass",
        },
        Unit {
            r#type: Time,
            name: "second",
            symbol: "s",
            measure: "time",
        },
        Unit {
            r#type: ElectricCurrent,
            name: "ampere",
            symbol: "A",
            measure: "electric current",
        },
        Unit {
            r#type: Temperature,
            name: "kelvin",
            symbol: "K",
            measure: "thermodynamic temperature",
        },
        Unit {
            r#type: Amount,
            name: "mole",
            symbol: "mol",
            measure: "amount of substance",
        },
        Unit {
            r#type: LuminousIntensity,
            name: "candela",
            symbol: "cd",
            measure: "luminous intensity",
        },
    );
}

fn main() {
    let (length, mass, time, current, temp, amount, lum) = init_base_units();

    let derived_measurements: Vec<DerivedMeasurement> = vec![
        DerivedMeasurement {
            name: "velocity",
            symbols: vec!["speed-of-light", "velocity"],
            units: vec![(&mass, 1), (&time, -1)],
        },
        DerivedMeasurement {
            name: "acceleration",
            symbols: vec!["acceleration"],
            units: vec![(&mass, 1), (&time, -2)],
        },
        DerivedMeasurement {
            name: "gravitational-constant",
            symbols: vec!["gravitational-constant"],
            units: vec![(&length, 3), (&mass, -1), (&time, -2)],
        },
        DerivedMeasurement {
            name: "momentum",
            symbols: vec!["momentum"],
            units: vec![(&mass, 1), (&length, 1), (&time, -1)],
        },
        DerivedMeasurement {
            name: "energy",
            symbols: vec!["energy"],
            units: vec![(&mass, 1), (&length, 2), (&time, -2)],
        },
        DerivedMeasurement {
            name: "power",
            symbols: vec!["power", "watt"],
            units: vec![(&mass, 1), (&length, 2), (&time, -3)],
        },
        DerivedMeasurement {
            name: "mass",
            symbols: vec!["mass", "kilogram"],
            units: vec![(&mass, 1)],
        },
        DerivedMeasurement {
            name: "length",
            symbols: vec!["meter"],
            units: vec![(&length, 1)],
        },
        DerivedMeasurement {
            name: "time",
            symbols: vec!["time", "second"],
            units: vec![(&time, 1)],
        },
        DerivedMeasurement {
            name: "current",
            symbols: vec!["current", "ampere"],
            units: vec![(&current, 1)],
        },
        DerivedMeasurement {
            name: "temperature",
            symbols: vec!["temperature"],
            units: vec![(&temp, 1)],
        },
        DerivedMeasurement {
            name: "amount",
            symbols: vec!["mole", "mol"],
            units: vec![(&amount, 1)],
        },
        DerivedMeasurement {
            name: "luminous intensity",
            symbols: vec!["luminous intensity", "candela", "luminosity"],
            units: vec![(&lum, 1)],
        },
    ];

    println!(
        "now finding equivalencies for {}",
        &derived_measurements[5].name
    );

    find_equivalencies(5, &derived_measurements);
    /*
       DerivedMeasurement_name: "c",
       units: vec![{m, 1}, {s, -1}]
    */

    println!("Hello, world!");
}

fn find_equivalencies(idx: usize, m: &Vec<DerivedMeasurement>) {}
