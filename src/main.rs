use std::collections::HashMap;
use std::io::{self, Write};

fn mean(list: &[i32]) -> f64 {
    let mut sum = 0.0;
    let mut len = 0.0;

    for &i in list {
        sum += f64::from(i);
        len += 1.0;
    }

    sum / len
}

fn median(list: &[i32]) -> i32 {
    list[list.len() / 2]
}

fn mode(list: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut count = 0;
    let mut num = None;

    for &i in list {
        *map.entry(i).or_insert(0) += 1;
    }

    for (n, c) in map {
        if c > count {
            count = c;
            num = Some(n);
        }
    }

    num.expect("list len must be > 0")
}

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
                println!("Result: {}", mean(&numbers));
            }
            "median" => {
                if numbers.len() == 0 {
                    continue;
                }
                numbers.sort_unstable();
                println!("Result: {}", median(&numbers));
            }
            "mode" => {
                if numbers.len() == 0 {
                    continue;
                }
                println!("Result: {}", mode(&numbers));
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
