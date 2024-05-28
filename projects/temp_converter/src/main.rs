use std::io;

fn main() {

    let mut converter = String::new();
    let mut value = String::new();

    println!("Please select converter:\n1. F to C\n2. C to F");
    io::stdin()
        .read_line(&mut converter)
        .expect("Failed to read line");

    let converter: u32 = converter.trim().parse().expect("Please type a number!");

    println!("Please enter a value to convert: ");
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: f32 = value.trim().parse().expect("Please type a number!");

    let unit = if converter == 1 { 'F' } else { 'C' };
    let opposite_unit = if converter == 1 { 'C' } else { 'F' };
    
    let computed: f32 = convert(unit, value);
    
    println!("{value}{unit} => {computed}{opposite_unit}");
}

fn convert(temp_unit: char, value: f32) -> f32 {

    if temp_unit == 'F' {
        let compute: f32 = value - 32.0;
        let result: f32 = compute * (5.0/9.0);
        return result;
    } else if temp_unit == 'C' {
        let compute: f32 = value * (9.0/5.0);
        let result: f32 = compute + 32.0;
        return result;
    }
    value
}
