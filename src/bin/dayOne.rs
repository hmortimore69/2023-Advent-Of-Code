use std::fs;

fn main() {
    let contents = fs::read_to_string("resources/dayOne/input.txt").expect("Something went wrong reading the file");
    let mut total = 0;
    let digit_map = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    for line in contents.lines() {
        let mut str_line = line.to_string();
        for (word, digit) in digit_map.iter() {
            let first_char = word.chars().next().unwrap(); 
            let last_char = word.chars().last().unwrap(); 

            let replacement = format!("{}{}{}", first_char, digit, last_char); 
            str_line = str_line.replace(word, &replacement); 
        }
        
        let digits: Vec<char> = str_line.chars().filter(|c| c.is_digit(10)).collect();

        if !digits.is_empty() {
            let first_digit = digits.first().unwrap();
            let last_digit = digits.last().unwrap();

            let calibration_value = format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
            
            
            total += calibration_value;
        }
    }

    println!("Total: {}", total);
}
