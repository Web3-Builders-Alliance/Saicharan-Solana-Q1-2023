pub fn run() {
    let age: u8 = 21;
    let check_id = true;
    let already_voted = true;

    // if else
    if age >= 18 && check_id && !already_voted {
        println!("eligible to vote");
    } else if age < 18 || check_id || already_voted {
        println!("not eligible to vote");
    } else if age >= 18 && !check_id {
        println!("need your id");
    } else {
        println!("hell no patience to write all conditions");
    }

    // shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("is of age: {}", is_of_age);
}