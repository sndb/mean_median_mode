use liststat::{mean, median, mode};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut numbers = Vec::new();

    println!("Available operations:");
    println!("mean, median, mode, clear, quit");

    loop {
        println!("Enter the next number or operation:");
        println!("Current numbers: {:?}", numbers);

        print!("$ ");

        io::stdout().flush()?;

        io::stdin().read_line(&mut input).unwrap();
        match input.to_lowercase().trim() {
            "mean" => {
                if numbers.len() == 0 {
                    continue;
                }
                println!("Result: {}", mean(&numbers).unwrap());
            }
            "median" => {
                if numbers.len() == 0 {
                    continue;
                }
                numbers.sort_unstable();
                println!("Result: {}", median(&numbers).unwrap());
            }
            "mode" => {
                if numbers.len() == 0 {
                    continue;
                }
                println!("Result: {}", mode(&numbers).unwrap());
            }
            "clear" => numbers.clear(),
            "quit" => {
                println!("Bye!");
                break Ok(());
            }
            n => {
                let n = n.parse();
                if let Ok(n) = n {
                    numbers.push(n);
                } else {
                    println!("You must enter a number.");
                }
            }
        }
        input.clear();
    }
}
