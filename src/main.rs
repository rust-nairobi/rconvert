#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json as json;
#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};

error_chain! {
    foreign_links {
        Network(reqwest::Error);
        Io(::std::io::Error);
        Json(json::Error);
    }
}

mod currency;
mod temperature;
mod weight;

use currency::{convert_currency};
use temperature::{convert_temperature};
use weight::{convert_weight};

fn main() {
    let matches = App::new("rconvert")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("currency")
                .about("Convert currency values.")
                .arg(Arg::with_name("units")
                    .help("Set currency units to convert from-to e.g usd-gbp")
                    .short("u")
                    .long("units")
                    .takes_value(true)
                    .required(true))
                .arg(Arg::with_name("value")
                    .help("Set currency value to convert e.g 100.00")
                    .short("v")
                    .long("value")
                    .takes_value(true)
                    .required(true))
        )
        .subcommand(
            SubCommand::with_name("temperature")
                .about("Convert temparature values.")
                .arg(Arg::with_name("units")
                    .help("Set temperature units to convert from-to e.g c-f")
                    .short("u")
                    .long("units")
                    .takes_value(true)
                    .required(true))
                .arg(Arg::with_name("value")
                    .help("Set temperature value to convert e.g 29.55")
                    .short("v")
                    .long("value")
                    .takes_value(true)
                    .required(true))
        )
        .subcommand(
            SubCommand::with_name("weight")
                .about("Convert weight values.")
                .arg(Arg::with_name("units")
                    .help("Set weight units to convert from-to e.g kg-lb")
                    .short("u")
                    .long("units")
                    .takes_value(true)
                    .required(true))
                .arg(Arg::with_name("value")
                    .help("Set weight value to convert e.g 85.55")
                    .short("v")
                    .long("value")
                    .takes_value(true)
                    .required(true))
        )
        .get_matches();

    match matches.subcommand() {
        ("currency", Some(sub_m)) => {
            let value: &str = sub_m.value_of("value").unwrap();
            let units: &str = sub_m.value_of("units").unwrap();
            let unit_elements: Vec<&str> = units.split("-").collect();

            if let Some(convert_from) = unit_elements.get(0) {
                if let Some(convert_to) = unit_elements.get(1) {
                    convert_currency(value, convert_from, convert_to);
                }
            }
        },
        ("temperature", Some(sub_m)) => {
            let value: &str = sub_m.value_of("value").unwrap();
            let units: &str = sub_m.value_of("units").unwrap();
            let unit_elements: Vec<&str> = units.split("-").collect();
            if let Some(convert_from) = unit_elements.get(0) {
                if let Some(convert_to) = unit_elements.get(1) {
                    convert_temperature(value, convert_from, convert_to);
                }
            }
        },
        ("weight", Some(sub_m)) => {
            let value: &str = sub_m.value_of("value").unwrap();
            let units: &str = sub_m.value_of("units").unwrap();
            let unit_elements: Vec<&str> = units.split("-").collect();
            if let Some(convert_from) = unit_elements.get(0) {
                if let Some(convert_to) = unit_elements.get(1) {
                    convert_weight(value, convert_from, convert_to);
                }
            }
        },
        _ => println!("Unsupported conversion command"),
    };
}
