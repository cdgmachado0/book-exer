use std::io;


fn main() {
    convert_to_celsius();

}




fn convert_to_celsius() {
    const FARENHEIT: f32 = 32.0;
    
    loop {
        let mut input = String::new();
        println!("Please insert a temperature in Fahrenheit");

        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<f32>() {
            Ok(input) => {
                let in_celsius = (input - FARENHEIT) * (5.0 / 9.0);
                println!("{input} degrees Farenheit is the same as {in_celsius} degrees Celsius");
            },
            Err(_) => continue,
        };
    }
}
