//
use std::env;
use std::string;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let name = "Jill";

    println!("Environment Args {:?}",args);

    let mut command = args[1].clone();
    println!("Command: {}",command);
    command.make_ascii_lowercase();

    if command == "hello" {
        println!("Hi {} how are you?",name);
    }
}