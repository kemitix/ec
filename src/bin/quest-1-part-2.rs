use std::{collections::HashMap, path::PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let file_path = PathBuf::from("./data/quest-1-part-2.txt");
    let data = std::fs::read_to_string(file_path)?;
    let counter = quest_part_2(&data);
    println!("Quest 1 / Part 2: {counter}");
    Ok(())
}

fn quest_part_2(data: &str) -> i32 {
    let table = HashMap::from([('A', 0), ('B', 1), ('C', 3), ('D', 5)]);
    let mut counter = 0;

    let mut is_left = false;
    let mut has_left_contender = false;
    let mut left_score: i32 = 0;
    for beast in data.chars() {
        is_left = !is_left;
        match (is_left, table.get(&beast)) {
            (true, None) => {
                // left has no contender
                left_score = 0;
                has_left_contender = false;
            }
            (true, Some(score)) => {
                left_score = *score;
                has_left_contender = true;
            }
            (false, None) => {
                // right has no container
                counter += left_score;
            }
            (false, Some(score)) if has_left_contender => {
                // has both left and right contender
                counter += left_score + score + 2;
            }
            (false, Some(score)) => {
                // has only right contender
                counter += score;
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::quest_part_2;

    #[test]
    fn sample() {
        let data = "AxBCDDCAxD";
        let result = quest_part_2(data);
        assert_ne!(result, 5835);
        assert_eq!(result, 28);
    }
}
