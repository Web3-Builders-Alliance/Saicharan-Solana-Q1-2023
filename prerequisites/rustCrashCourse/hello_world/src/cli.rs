use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = args[2].clone();
    println!("command: {:?}", command);

    if command == "hello" {
        println!("hi there, {}, how are you?", name);
    } else if command == "bye" {
        println!("see you soon!, {}.", name);
    }
}