use std::fs;
use std::collections::HashMap;

fn main() {
    let file = fs::read_to_string("resources/dayTwo/data.txt")
        .expect("Something went wrong reading the file");

    let mut total = 0;
    let mut power_factors: u32 = 0;

    const RED_MAX: usize = 100; // Part One -> Set to 12
    const GREEN_MAX: usize = 100; // Part Two -> Set to 13
    const BLUE_MAX: usize = 100; // Part Three -> Set to 14

    for line in file.lines() {
        let mut line_split: Vec<&str> = Vec::new();
        let mut min_count: HashMap<&str, usize> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        let game_id = match line[5..(line.find(':').unwrap())].parse::<usize>() {
            Ok(id) => id,
            Err(_) => {
                println!("Error parsing game ID, skipping line: {}", line);
                continue;
            }
        };

        let ball_counts: Vec<&str> = line.split(": ").last().unwrap().split("; ").collect(); 
        line_split.extend(ball_counts.iter().flat_map(|item| item.split(", ")));

        let mut line_processed = true;
        for item in line_split {
            let (number, color) = if let [num, col] = item.split_whitespace().collect::<Vec<_>>()[..] {
                (num.parse::<usize>().unwrap_or_else(|_| {
                    line_processed = false;
                    println!("Error parsing number, skipping item: {}", item);
                    0
                }), col)
            } else {
                line_processed = false;
                continue;
            };

            let (color_max, _color_name) = match color {
                "red" => (RED_MAX, "red"),
                "green" => (GREEN_MAX, "green"),
                "blue" => (BLUE_MAX, "blue"),
                _ => {
                    line_processed = false;
                    println!("Unknown color, skipping item: {}", item);
                    continue;
                }
            };

            if number > *min_count.get(color).unwrap() {
                min_count.insert(color, number);
            }

            if number > color_max {
                line_processed = false;
                break;
            }
        }

        if line_processed {
            total += game_id;
            let temp: u32 = (min_count["red"] * min_count["green"] * min_count["blue"]) as u32;
            println!("Game ID: {}, Possible: {}, Min: (R: {}, G: {}, B: {}), Power: {}", game_id, line_processed, min_count["red"], min_count["green"], min_count["blue"], temp);
            power_factors += temp;
        }
    }

    println!("Total: {}, Power: {:?}", total, power_factors);
}