// cargo-deps: regex

extern crate regex;

use std::io;
use regex::Regex;

const WEIGHT: f32 = 1.8;
const BIAS: u32 = 32;

fn main() {
    println!("Fahrenheit and Celsius");

    println!("Input temperature:");
    println!("e.g
  Input 1: 100C
  Output 1: 212.0F
  Input 2: 100F
  Output 2: 37.7778C");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp)
        .expect("Failed to readline");

    let unit = temp.split_off(temp.len() - 2);
    let temp = temp.trim().parse()
        .expect("Not a number");

    println!("temp: {:?}, unit: {:?}", temp, unit);

    let re_fahrenheit = Regex::new(r"[F|f]").unwrap();
    let re_celsius = Regex::new(r"[C|c]").unwrap();

    let converted = if re_fahrenheit.is_match(&*unit) {
        (celsius(temp, WEIGHT, BIAS as f32), "°C")
    } else if re_celsius.is_match(&*unit) {
        (fahrenheit(temp, WEIGHT, BIAS as f32), "°F")
    } else {
        (0.0, "--ERROR")
    };

    println!("{:.2}{}", converted.0, converted.1);
}

fn fahrenheit(c: f32, weight: f32, bias: f32) -> f32 {
    weight * c + bias
}

fn celsius(f: f32, weight: f32, bias: f32) -> f32 {
    (f - bias) / weight
}
