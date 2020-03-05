/// Information relating to a person
struct Person {
    first_name : String,
    last_name : String,
    gender : String,
    age : usize,
    home_address : String,
    home_phone_number : String,
    number_of_arms : usize,
}

fn main() {
    let father = Person {
        first_name : String::from("John"),
        last_name : String::from("Smith"),
        gender : String::from("M"),
        age : 55,
        home_address : String::from(
            "3rd floor, Broers Building,\n21 JJ Thomson Ave,\nCambridge CB3 0FA"
        ),
        home_phone_number : String::from("07123-456-789"),
        number_of_arms : 2,
    };

    let daughter = Person {
        first_name : String::from("Jane"),
        gender : String::from("F"),
        age : 20,
        ..father
    };

    println!("{} {}'s address:\n{}", daughter.first_name, daughter.last_name, daughter.home_address);

    // Is this allowed?
    //println!("{} {}'s address:\n{}", father.first_name, father.last_name, father.home_address);


    println!("\n\n{} has {} arms.", daughter.first_name, daughter.number_of_arms);
    
    // Is this allowed?
    //println!("{} has {} arms.", father.first_name, father.number_of_arms);
}
