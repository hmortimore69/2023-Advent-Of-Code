use std::fs;

fn main() {
    let file = fs::read_to_string("resources/dayTwo/data.txt")
        .expect("Something went wrong reading the file");

    let mut total = 0;

    const RED_MAX: usize = 12;
    const GREEN_MAX: usize = 13;
    const BLUE_MAX: usize = 14;

    for line in file.lines() {
        let mut line_split: Vec<&str> = Vec::new();

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

            if number > color_max {
                line_processed = false;
                break;
            }
        }

        println!("Game ID: {}, Possible: {}", game_id, line_processed);
        if line_processed {
            total += game_id;
        }
    }

    println!("Total: {}", total);
}