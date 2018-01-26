#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate serde_json as json;
extern crate rconvert;

use clap::{Arg, App, SubCommand, ArgMatches};
use rconvert::currency::{convert_currency};
use rconvert::temperature::{convert_temperature};
use rconvert::weight::{convert_weight};

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
            let (value, from, to) = get_conversion_options(sub_m);
            convert_currency(&value, &from, &to);
        },
        ("temperature", Some(sub_m)) => {
            let (value, from, to) = get_conversion_options(sub_m);
            convert_temperature(&value, &from, &to);
        },
        ("weight", Some(sub_m)) => {
            let (value, from, to) = get_conversion_options(sub_m);
            convert_weight(&value, &from, &to);
        },
        _ => println!("Unsupported conversion command"),
    };
}

fn get_conversion_options(sub_m: &ArgMatches) -> (String, String, String) {
    let value: String = sub_m.value_of("value").unwrap().to_owned();
    let units: String = sub_m.value_of("units").unwrap().to_owned();
    let unit_elements: Vec<&str> = units.split("-").collect();
    assert!(unit_elements.len() == 2);
    let from: String = unit_elements[0].to_owned();
    let to: String = unit_elements[1].to_owned();
    (value, from, to)
}
