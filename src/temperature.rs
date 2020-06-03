const WEIGHT: f32 = 1.8;
const BIAS: u32 = 32;

const F: &str = "F";
const C: &str = "C";
const DEG_F: &str = "°F";
const DEG_C: &str = "°C";

fn main() {
    println!("Fahrenheit and Celsius");

    println!("Input temperature:");
    println!("e.g
  Input 1: 100{C:}
  Output 1: 212.0{F:}
  Input 2: 100{F:}
  Output 2: 37.7778{C:}",
    C=C, F=F
    );

    let mut temp: String = read();
    let (temp, unit): (&str, &str) = temp.split_at(temp.len() - 1);
    let temp: f32 = temp.parse().expect("Not a number");

    let converted = match &*(unit.to_uppercase()) {
        "F" => (celsius(temp, WEIGHT, BIAS as f32), DEG_C),
        "C" => (fahrenheit(temp, WEIGHT, BIAS as f32), DEG_F),
        _ => (0.0, "--ERROR"),
    };

    println!("{:.2}{}", converted.0, converted.1);
}

fn fahrenheit(c: f32, weight: f32, bias: f32) -> f32 {
    weight * c + bias
}

fn celsius(f: f32, weight: f32, bias: f32) -> f32 {
    (f - bias) / weight
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
