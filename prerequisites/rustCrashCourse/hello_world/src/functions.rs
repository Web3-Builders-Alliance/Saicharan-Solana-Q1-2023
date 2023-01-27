pub fn run() {
    greeting("hey there", "saicharan");

    // bind function values to variables
    let sum = add(21, 2);
    println!("sum: {}", sum);

    // closure
    let n3 = 12;
    let add_numbers = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("c sum: {}", add_numbers(3, 9));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}