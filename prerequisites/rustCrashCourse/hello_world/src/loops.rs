#[allow(dead_code)]
fn infinite() {
    let mut count = 0;
    // infinite loop
    loop {
        count += 1;
        println!("number: {}", count);

        if count == 20 {
            break;
        }
    }
}

#[allow(dead_code)]
fn while_loop() {
    let mut count = 0;
    // white loop
    while count <= 50 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("buzz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
        // increment count
        count += 1;
    }
}

#[allow(dead_code)]
fn for_range() {
    // for range
    for x in 0..51 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("buzz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }
}

pub fn run() {
    // infinite();
    println!("commented infinite loop");
    // while_loop();
    println!("commented while loop");
    // for_range();
    println!("commented for-range loop");
}