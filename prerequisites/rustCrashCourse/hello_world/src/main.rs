mod print;
mod vars;

use colored::*;

fn main() {
    println!("\n{}", "println:".yellow());
    print::run();
    println!("\n{}", "variables:".yellow());
    vars::run();
}