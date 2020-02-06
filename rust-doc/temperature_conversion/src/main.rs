use std::io;

fn main() {
    // Base 
    println!("Base (Fahrenheit: 1 , Celsius: 2) -->");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)
        .expect("Faild to read line");
    
    let base: u32 = match buf.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    
    //Value
    let mut buf = String::new();
    print!("value : ");
    println!("");
    io::stdin().read_line(&mut buf)
        .expect("Faild to read line");

    let value: f32 = match buf.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };


    if base == 1 {
        //  Fahrenheit to Celsius
        let result = (5.0 / 9.0) * (value - 32.0);
        println!("Fahrenheit to Celsius: {} ==> {}", value, result);
    } else {
        // Celsius to Fahrenheit
        let result = (9.0 / 5.0) * (value + 32.0);
        println!("Celsius to Fahrenheit: {} ==> {}", value, result);
    }
}