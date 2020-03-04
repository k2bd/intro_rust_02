enum Gender {
    Male,
    Female,
    Other(String),
}

/// Information relating to a person
struct Person {
    first_name : String,
    last_name : String,
    gender : Gender,
    age : usize,
    home_address : String,
    home_phone_number : String,
}

fn main() {
    let mut father = Person {
        first_name : String::from("John"),
        last_name : String::from("Smith"),
        gender : Gender::Male,
        age : 55,
        home_address : String::from(
            "3rd floor, Broers Building,\n21 JJ Thomson Ave,\nCambridge CB3 0FA"
        ),
        home_phone_number : String::from("07123-456-789"),
    };

    let mut daughter = Person {
        first_name : String::from("Jane"),
        gender : Gender::Female,
        age : 20,
        ..father
    };

    // NOT allowed: use of moved value
    //father.home_address.push_str(",\n United Kingdom");
    
    // NOT allowed: use of moved value
    //println!("Hello {} {}", father.first_name, father.last_name);

    // Are these allowed?
    father.first_name.push_str("athan");
    println!("Hello {}", father.first_name);
}
