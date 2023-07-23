use std::io;


fn main() {
    convert_to_celsius();

}




fn convert_to_celsius() {
    const FARENHEIT: f32 = 33.8;
    
    loop {
        println!("Please insert a temperature in Fahrenheit");
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<f32>() {
            Ok(num) => {
                let in_celsius = num / FARENHEIT;
                println!("{num} degrees Farenheit is the same as {in_celsius} degrees Celsius");
                break;
            },
            Err(_) => continue,
        };
    }
}
