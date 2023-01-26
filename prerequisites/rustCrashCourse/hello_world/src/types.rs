pub fn run() {
    // default is i32
    let _x = 1;

    // default is f64
    let _y = 2.5;

    // add explicit type
    let _z: i64 = 1234567890;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // boolean
    let is_active = true;

    // gen boolean from expression
    let is_greater: bool = 10 > 11;

    // char
    let character = 's';

    // unicode
    let solana = '\u{1F525}';

    println!("{:?} {} {} {}", if is_active { true } else { false }, is_greater, character, solana);
}