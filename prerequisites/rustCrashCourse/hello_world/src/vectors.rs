use std::mem::*;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", numbers);

    // re-assign value
    numbers[0] = 6;
    println!("{:?}", numbers);

    // add 
    numbers.push(7);
    println!("pushed {:?}", numbers);

    // pop off last value
    numbers.pop();
    println!("pushed {:?}", numbers);

    // get single value
    println!("single value: {}", numbers[0]);
    
    // vector length
    println!("vector length: {}", numbers.len());

    // vector are stack allocated 
    println!("vector occupies {} bytes", size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..3];
    println!("slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("number: {:?}", numbers);
}