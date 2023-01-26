pub fn run() {
    let name = "saicharan";
    let mut age = 23;
    println!("my name is {} & i'm {}", name, age);
    age = 23 + 1;
    println!("today was my birthday & i'm now {}", age);

    // define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let (my_name, my_age) = ("saicharan", 24);
    println!("{} is {}!", my_name, my_age);
}