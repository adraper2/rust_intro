// this is a rust comment

// how to use packages
use std::io;
use rand::Rng;

fn main() {
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
