use reqwest;
use reqwest::Response;
use std;
use json;

#[derive(Deserialize, Debug)]
struct ExchangeRate {
    base: String,
    date: String,
    rates: json::Value,
}

#[derive(Debug)]
enum CustomError {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    RespStatusError,
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> CustomError {
        CustomError::Io(err)
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError::Reqwest(err)
    }
}

pub fn convert_currency(amount: &str, from: &str, to: &str) {
    let currency_json: json::Value = match get_currency_exchange(from, to) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("error: {:?}", e);
            std::process::exit(1);
        },
    };
    println!("Exchnage rate is {:?}", currency_json);
}

fn get_currency_exchange(from: &str, to: &str) -> Result<json::Value, CustomError> {
    let url = format!("https://api.fixer.io/latest?symbols={},{}",
        from.to_uppercase(),
        to.to_uppercase());
    let mut resp = send_request(&url)?;
    if resp.status().as_u16() == 200 {
        let data: json::Value = resp.json()?;
        let rates: json::Value = data.get("rates").unwrap().clone();
        Ok(rates)
    } else {
        Err(CustomError::RespStatusError)
    }
}

fn send_request(url: &str) -> Result<Response, CustomError> {
    let client = reqwest::Client::new();
    let resp = client.get(url).send()?;
    Ok(resp)
}
