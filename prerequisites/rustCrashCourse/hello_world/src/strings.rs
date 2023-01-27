pub fn run() {
    // primitive str
    let immutable_str = "sai";
    println!("length: {}", immutable_str.len());
    
    // owned string
    let new_str = immutable_str.to_owned() + "charan";
    println!("length of {}: {}", new_str, new_str.len());

    // String
    let mut name = String::from("sai");

    // get length
    println!("length: {}", name.len());

    // push char
    name.push('c');

    // push str
    name.push_str("haran");
    println!("{}", name);

    // capacity in bytes
    println!("capacity: {}", name.capacity());

    // check is empty
    println!("is empty: {}", name.is_empty());

    // contains
    println!("contains 'sai' {}", name.contains("sai"));

    // replace
    println!("replace: {}", name.replace("sai", "Sai"));

    name.push_str(" pogul");
    // loop through string by whitespace
    for word in name.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('s');
    s.push('a');
    s.push('i');

    // assertion testing
    assert_eq!(3, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}