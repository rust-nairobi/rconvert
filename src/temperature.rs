struct Temperature {
    unit: String,
    celsius_conversion_rate: f32,
}

pub fn convert_temperature(value: &str, from: &str, to: &str) {
    let celsius = Temperature {
        unit: String::from("celsius"),
        celsius_conversion_rate: 1.0,
    };

    let fahrenheit = Temperature {
        unit: String::from("fahrenheit"),
        celsius_conversion_rate: 33.8,
    };

    let kelvin = Temperature {
        unit: String::from("kelvin"),
        celsius_conversion_rate: 274.8,
    };

    let result = match (from,to) {
        ("c","f") => {
            "Converting c to f"
        },
        ("f","c") => {
            "Converting f to c"
        },
        ("c","k") => {
            "Converting c to k"
        },
        ("k","c") => {
            "Converting k to c"
        },
        ("k","f") => {
            "Converting k to f"
        },
        ("f","k") => {
            "Converting f to k"
        },
        _ => "Unsupported temperature units"
    };

    println!("{}", result);
}
