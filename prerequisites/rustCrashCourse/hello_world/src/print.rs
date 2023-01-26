pub fn run() {
    // print to console
    println!("hello from print.rs file.");

    // basic formatting 
    println!("{} is from {}", "saicharan", "india");

    // positional arguments
    println!("{0} is from {1} & {0} loves to {2}", "saicharan", "india", "code");

    // named arguments
    println!("{name} loves {community}", name = "saicharan", community = "solana");

    // placeholder traits
    println!("binary: {:b} \nHex: {:x}\nOctal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (1, true, "solana", ["here", "we", "go"]));

    // basic math
    println!("10.3 + 10.7 = {}", 10.3+10.7);
}