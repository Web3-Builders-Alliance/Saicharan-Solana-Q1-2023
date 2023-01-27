mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;

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
}