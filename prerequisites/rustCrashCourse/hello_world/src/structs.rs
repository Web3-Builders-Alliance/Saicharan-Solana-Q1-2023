// traditional struct
struct ColorV1 {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct ColorV2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = ColorV1 {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 120;
    println!("color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = ColorV2(255, 0, 0);
    c2.0 = 200;
    println!("color: {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("saicharan", "pogu");
    println!("person: {} {}", p.first_name, p.last_name);
    p.set_last_name("pogul");
    println!("person full name: {}", p.full_name());
    println!("person in tuple: {:?}", p.to_tuple());
}