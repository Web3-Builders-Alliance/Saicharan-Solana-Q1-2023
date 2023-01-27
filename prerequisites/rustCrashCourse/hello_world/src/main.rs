mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointers;
mod structs;
mod enums;

use colored::*;

fn main() {
    println!("\n{}", "println:".yellow());
    print::run();
    println!("\n{}", "variables:".yellow());
    vars::run();
    println!("\n{}", "types:".yellow());
    types::run();
    println!("\n{}", "strings:".yellow());
    strings::run();
    println!("\n{}", "tuples:".yellow());
    tuples::run();
    println!("\n{}", "arrays:".yellow());
    arrays::run();
    println!("\n{}", "vectors:".yellow());
    vectors::run();
    println!("\n{}", "conditionals:".yellow());
    conditionals::run();
    println!("\n{}", "loops:".yellow());
    loops::run();
    println!("\n{}", "functions:".yellow());
    functions::run();
    println!("\n{}", "pointers:".yellow());
    pointers::run();
    println!("\n{}", "structs:".yellow());
    structs::run();
    println!("\n{}", "enums:".yellow());
    enums::run();
}