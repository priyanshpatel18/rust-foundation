use std::fs::read_to_string;

fn main() {
    let ans = read_from_file_kirat(String::from("a.txt"));
    println!("{:?}", ans);
}

fn read_from_file_kirat(file_path: String) -> Result<String, String> {
    let result = read_to_string(file_path);
    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(String::from(err.to_string())),
    }
}
