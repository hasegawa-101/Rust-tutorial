use std::io;

fn main() {
    loop {
        println!("Convert celsius to fahrenheit? y/n");

        let mut is_convert_fahrenheit = String::new();
        io::stdin().read_line(&mut is_convert_fahrenheit).expect("Failed to read line");

        match is_convert_fahrenheit.trim() {
            "y" => {
                println!("Please input celsius");
                let mut celsius = String::new();
                io::stdin().read_line(&mut celsius).expect("Failed to read line");

                let celsius: f64 = celsius.trim().parse().unwrap();
                let fahrenheit: f64 = (celsius * 1.8) + 32.0;
                println!("{}F", fahrenheit);
            },
            "n" => {
                println!("Please input fahrenheit");
                let mut fahrenheit = String::new();
                io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");

                let fahrenheit: f64 = fahrenheit.trim().parse().unwrap();
                let celsius: f64 = (fahrenheit - 32.0) / 1.8;
                println!("{}C", celsius);
            },
            _ => {
                println!("Please input y or n");
                continue;
            },
        };
    }
}
