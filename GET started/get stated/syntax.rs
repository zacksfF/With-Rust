fn multiply(x: i64, y: i64) -> i64 {
    x * y
}

fn main() {
    let z = multiply(5, 6);
    println!("result: {}", z);
    let txt = format!("result: {z}", z=z);
    println!("{}", txt);
}
