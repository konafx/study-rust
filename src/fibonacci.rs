fn main() {
    println!("Fibonacci number");
    let mut n: u32 = read();
    println!("F_{}: {}", n, fibonacci(n));
    println!(" MAX: {}", u32::max_value());
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n-2) + fibonacci(n-1),
    }
}
