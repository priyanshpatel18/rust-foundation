use std::collections::HashMap;

fn main() {
    let mut users: HashMap<String, i32> = HashMap::new();

    users.insert(String::from("Priyansh"), 20);
    users.insert(String::from("Mom"), 45);
    users.insert(String::from("Dad"), 50);

    let my_age = users.get("Priyansh");
    println!("{:?}", &my_age);

    println!("{:?}", users);

    let map = assignment(vec![
        (String::from("Priyansh"), 20),
        (String::from("Mom"), 45),
        (String::from("Dad"), 50),
    ]);
    println!("{:?}", map);
}

// Assignment
// Write a function that takes a vector of tuples and returns a HashMap
fn assignment(tuples: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();

    for (key, value) in tuples {
        map.insert(key, value);
    }

    return map;
}
