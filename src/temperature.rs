use std;

pub fn convert_temperature(value: &str, from: &str, to: &str) {
    let value: f32 = match value.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("error: {:?}", e);
            std::process::exit(1);
        },
    };

    let result = match (from, to) {
        ("c","f") => {
            let converted_amount: f32 = (value * 9.0/5.0) + 32.0;
            String::from(format!("{}{} to {}\n{}{}", value, from, to, converted_amount, to))
        },
        ("f","c") => {
            let converted_amount: f32 = (value - 32.0) * 5.0/9.0;
            String::from(format!("{}{} to {}\n{}{}", value, from, to, converted_amount, to))
        },
        ("c","k") => {
            let converted_amount: f32 = value + 273.15;
            String::from(format!("{}{} to {}\n{}{}", value, from, to, converted_amount, to))
        },
        ("k","c") => {
            let converted_amount: f32 = value - 273.15;
            String::from(format!("{}{} to {}\n{}{}", value, from, to, converted_amount, to))
        },
        ("k","f") => {
            let converted_amount: f32 = (value - 273.15) * (9.0/5.0) + 32.0;
            String::from(format!("{}{} to {}\n{}{}", value, from, to, converted_amount, to))
        },
        ("f","k") => {
            let converted_amount: f32 = (value - 32.0) * (5.0/9.0) + 273.15;
            String::from(format!("{}{} to {}\n{}{}", value, from, to, converted_amount, to))
        },
        _ => String::from("Unsupported temperature units")
    };

    println!("{}", result);
}
