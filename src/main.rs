use std::io;


fn main() {
    // convert_to_celsius();
    // get_fibonacci();
    christmas_song();
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


fn _get_fibonacci() { 
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


fn christmas_song() {
    let days = ["first", "second", "third", "fourth", "fith", "sixth", "seventh", "eith", "nineth", "tenth", "eleventh", "twelfth"];
    let amounts = ["", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"];
    let elements = [
        "A partridge in a pear tree",
        "turtle doves, and",
        "french hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming"
    ];
    let mut verse: String = Default::default();

    for (i, day) in days.iter().enumerate() {

        let opening = format!("On the {} day of Christmas, my true love sent to me\n", day);

        if i == 0 {
            verse += &(format!("{}{}", elements[i], "\n"));
        } else {
            verse += &(format!("{} {}", amounts[i], elements[i]));
        }

        let verse = format!("{}{}{}", opening, verse, "\n\n");
        println!("{verse}");
    }




}