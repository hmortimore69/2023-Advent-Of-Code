use std::fs;

fn main() {
    let contents = fs::read_to_string("resources/dayTwo/data.txt")
        .expect("Something went wrong reading the file");
}
