use std::io;

fn main() {
    println!("Enter the temperature in fahrenheit:");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: i32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            return
        },
    };

    let celcius: i32 = (temp - 32) * 5/9;

    println!("The temperature in celcius is: {celcius}");
}
