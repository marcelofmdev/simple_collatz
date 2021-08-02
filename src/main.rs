// Collatz simplão de terminal

use std::io::{self, Write};
use std::{thread, time};

fn main() {
    println!("SIMPLE COLLATZ");
    print!("Choose a seed: ");
    io::stdout().flush().unwrap();

    let mut seed = String::new();

    io::stdin()
    .read_line(&mut seed)
    .expect("Failed to read line");

    let seed: i64 = match seed.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    if seed != 0 {
        let mut i = seed;
        let mut sequence = vec![i];
        //println!("{}", i);

        // Populating sequence vector
        loop {
            if i == 1 {
                break;
            }

            if i % 2 == 1 {
                i = (i*3) + 1;
                
            } else {
                i = i/2;
                
            }

            sequence.push(i)
        }

        // Figuring out the scale to plot
        let mut max = seed;
        for i in &sequence {
            if *i > max {
                max = *i;
            }
        }

        let scale = max;

        // Plotting sequence
        for i in &sequence {
            let resto: i64 = ((*i * 99)/scale) + 1;
            //let mut s = String::new();
            for _ in 0..resto {
                print!(".");
                io::stdout().flush().unwrap();
                let delay_time = time::Duration::from_millis(2);
                thread::sleep(delay_time);
            }

            println!(" {}", *i);

            let delay_time = time::Duration::from_millis(10);
            thread::sleep(delay_time);
        }
    }
}
