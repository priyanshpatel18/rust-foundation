// cargo add chrono
use chrono::Utc;

fn main() {
    let now = Utc::now();
    println!("current time is {}", now);
}
