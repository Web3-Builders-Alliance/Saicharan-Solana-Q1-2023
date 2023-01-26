mod print;
mod vars;
mod types;

use colored::*;

fn main() {
    println!("\n{}", "println:".yellow());
    print::run();
    println!("\n{}", "variables:".yellow());
    vars::run();
    println!("\n{}", "types:".yellow());
    types::run();
}