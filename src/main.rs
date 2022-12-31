// this is a rust comment

// how to use packages
use clap::Parser;
use std::io;
use rand::Rng;
use serde_json::{Result, Value};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    // get user input at command line
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    
    let args = Cli {
    pattern: pattern,
    path: std::path::PathBuf::from(path),
    };

    let _print_times = rand::thread_rng().gen_range(1..5);
    println!("Hello, world! Write something below:");
    
    let mut user_input =  String::new();

    io::stdin().read_line(&mut user_input).expect("Read line failed");
    
    let mut count = 0;

    loop{
        count +=1;
        // print user input this many times
        if count == _print_times {
            println!("Ok, enough...");
            break;
        }
        println!("You said {user_input}");

    }
}

fn append_new_content() -> Result<()> 
