use xpenser::Args;
use clap::Parser;
use std::{process, fs};
use xpenser::{utils, ExpenseTracker};

fn main() {
    let args = Args::parse();    
    let file_name = "expenses.json";

    if fs::exists(file_name).expect("Check file existence failed") {
        let file_content = utils::read_file_string(file_name).unwrap_or_else(|e| {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        });

        let mut tracker: ExpenseTracker = serde_json::from_str(&file_content).unwrap_or_else(|e| {
            eprintln!("Couldn't parse json to struct: {}", e);
            process::exit(1);
        });

        let result = tracker.process_action(args.action);
        println!("{}", result);

        let json = serde_json::to_string(&tracker).unwrap_or_else(|e| {
            eprintln!("Couldn't parse struct to json: {}", e);
            process::exit(1);
        });

        utils::create_file_write_all(file_name, json.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Error while creating and writing file: {}", e);
            process::exit(1);
        });
    } else {
        let mut tracker = ExpenseTracker::new();
        
        let result = tracker.process_action(args.action);
        println!("{}", result);

        let json = serde_json::to_string(&tracker).unwrap_or_else(|e| {
            eprintln!("Couldn't parse struct to json: {}", e);
            process::exit(1);
        });

        utils::create_file_write_all(file_name, json.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Error while creating and writing file: {}", e);
            process::exit(1);
        });
    }
}
