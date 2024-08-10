use std::env;
use std::fmt::Error;
use std::fs;
struct data {
    my_path: Vec<String>,
}
impl data {
    fn filter(&self) -> Result<String, String> {
        if self.my_path.len() != 3 {
            return Err("Invalid number of arguments".to_string());
        }
        let numm = self.my_path[2].clone();
        let b = match numm.parse::<i32>() {
            Ok(n) => return Ok(n.to_string()),
            Err(e) => return Err("Error parsing".to_string()),
        };
    }
}
fn main() {
    let a = data {
        my_path: env::args().collect(),
    };

    let b = a.filter();
    match b {
        Ok(b) => println!("{}", b.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()),
        Err(e) => println!("{}", e),
    }
}
