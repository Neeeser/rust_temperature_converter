use std::env;
use std::io;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 3 {
        let unit: f32 = args[2].trim().parse().expect("Not a number");

        match args[1].as_str() {
            "-c" => println!("In fahrenhiet: {}", cel_to_fah(unit)),
            "-f" => println!("In celsius: {}", fah_to_cel(unit)),
            _ => println!("Usage is -c or -f"),
        }
    } else {
        let mut input = String::new();
        println!("Enter a number to convert it both to Celcius and Farhenhiet");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input: f32 = input.trim().parse().expect("Not a number");

        println!("In Fahrenhiet: {}", cel_to_fah(input));
        println!("In Celsius: {}", fah_to_cel(input));
    }
}

fn cel_to_fah(cel: f32) -> f32 {
    1.8 * cel + 32.0
}

fn fah_to_cel(fah: f32) -> f32 {
    (fah - 32.0) / 1.8
}
