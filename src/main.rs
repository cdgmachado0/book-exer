use std::io;


fn main() {
    // convert_to_celsius();

    get_fibonacci();

}




fn _convert_to_celsius() {
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


fn get_fibonacci() { 
    'outter: loop {
        println!("What fibonacci number are you interested in?");
        let mut input = String::new();
        let mut i = 0;
        let mut prev1: u64;
        let mut prev2 = 1;
        let mut fib_num: u64 = 1;

        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u64>() {
            Ok(input) => {
                if input == 0 {
                    println!("Please input a number different than 0");
                    continue 'outter;
                }

                let n_num = loop {
                    if i == 0 {
                        prev1 = 0;
                        prev2 = 1;
                    } else {
                        prev1 = prev2;
                        prev2 = fib_num;
                    }
            
                    fib_num = prev1 + prev2;
            
                    i += 1;
            
                    if input == 2 {
                        break prev2;
                    } else if input == 1 {
                        break prev1;
                    } else if i == input - 2 {
                        break fib_num;
                    }
            
                    continue;
                };

                print!("The Nth fibonacci number is: {n_num}\n");
                continue 'outter;
            },
            Err(_) => {
                println!("Please input a number");
                continue;
            },
        };
    };


}
