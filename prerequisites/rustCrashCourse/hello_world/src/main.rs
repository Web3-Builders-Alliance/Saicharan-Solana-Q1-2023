mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;

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
}