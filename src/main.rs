use std::io;


fn main() {
    convert_to_celsius();

}




fn convert_to_celsius() {

    println!("Please insert a temperature in Fahrenheit");

    let mut temp = String::new();
    const farenheit: f32 = 33.8;

    io::stdin()
        .read_line(&mut temp)
        .expect("Please input a number");

    let temp: f32 = temp.trim().parse().unwrap();
    let in_celsius = temp / farenheit;

    println!("{temp} degrees Farenheit is the same as {in_celsius} degrees Celsius");

}
