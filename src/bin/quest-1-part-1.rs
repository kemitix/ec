use std::{collections::HashMap, path::PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let file_path = PathBuf::from("./data/quest-1-part-1.txt");
    let data = std::fs::read_to_string(file_path)?;
    let counter = quest_part_1(&data);
    println!("Quest 1 / Part 1: {counter}");
    Ok(())
}

fn quest_part_1(data: &str) -> i32 {
    let table = HashMap::from([('A', 0), ('B', 1), ('C', 3)]);
    let mut counter = 0;
    for beast in data.chars() {
        if let Some(potions) = table.get(&beast) {
            counter += potions;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::quest_part_1;

    #[test]
    fn sample() {
        let data = "ABBAC";
        let result = quest_part_1(data);
        assert_eq!(result, 5);
    }
}
