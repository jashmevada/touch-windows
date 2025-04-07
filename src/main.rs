use std::fs::{OpenOptions, metadata};
use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let file_path = &args[1];

        match metadata(file_path) {
            Ok(_) => println!("The file already exists. No changes made."),
            Err(_) => {
                match OpenOptions::new().write(true).create_new(true).open(file_path) {
                    Ok(_) => println!("The file is created successfully."),
                    Err(_) => {
                        println!("The file could not be opened or created. The program will exit now.");
                        process::exit(0);
                    }
                }
            }
        }
    } else {
        println!("Usage: touch <file_path>");
    }
}
