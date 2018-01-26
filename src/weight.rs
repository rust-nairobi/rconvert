pub fn convert_weight(value: &str, convert_from: &str, convert_to: &str) {
    let s = format!("Weight: Convert {}{} -> {}", value, convert_from, convert_to);
    println!("{}", s);
}
