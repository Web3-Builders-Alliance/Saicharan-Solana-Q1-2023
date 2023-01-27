use std::mem::*;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);

    // re-assign value
    numbers[0] = 6;
    println!("{:?}", numbers);

    // get single value
    println!("single value: {}", numbers[0]);
    
    // array length
    println!("array length: {}", numbers.len());

    // array are stack allocated 
    println!("array occupies {} bytes", size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..3];
    println!("slice: {:?}", slice);
}