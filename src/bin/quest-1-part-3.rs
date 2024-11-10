use std::{collections::HashMap, path::PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let file_path = PathBuf::from("./data/quest-1-part-3.txt");
    let data = std::fs::read_to_string(file_path)?;
    let counter = quest_part_3(&data);
    println!("Quest 1 / Part 3: {counter}");
    Ok(())
}

#[derive(PartialEq, Eq)]
enum Position {
    Left,
    Middle,
    Right,
}
#[derive(Debug, Default)]
struct Fight {
    left: i32,
    middle: i32,
    right: i32,
    combatants: i32,
}

fn quest_part_3(data: &str) -> i32 {
    let table = HashMap::from([('A', 0), ('B', 1), ('C', 3), ('D', 5)]);
    let mut counter = 0;

    let mut position = Position::Left;
    let mut fight = Fight::default();
    for beast in data.chars() {
        match (&position, table.get(&beast)) {
            (Position::Left, None) => {
                fight.left = 0;
                fight.combatants = 0;
            }
            (Position::Left, Some(score)) => {
                fight.left = *score;
                fight.combatants = 1;
            }
            (Position::Middle, None) => {
                fight.middle = 0;
            }
            (Position::Middle, Some(score)) => {
                fight.middle = *score;
                fight.combatants += 1;
            }
            (Position::Right, None) => {
                fight.right = 0;
            }
            (Position::Right, Some(score)) => {
                fight.right = *score;
                fight.combatants += 1;
            }
        }
        if position == Position::Right {
            let score = fight.left + fight.middle + fight.right;
            let bonus = match fight.combatants {
                3 => 6, // two per combatant
                2 => 2, // one per combatant
                1 => 0, // no bonus
                _ => 0, // fail-safe
            };
            counter += score + bonus;
            println!("{fight:?} - {score} - {bonus} -> {} --> {counter}", score + bonus);
        }
        position = match &position {
            Position::Left => Position::Middle,
            Position::Middle => Position::Right,
            Position::Right => Position::Left,
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::quest_part_3;

    #[test]
    fn sample() {
        let data = "xBxAAABCDxCC";
        let result = quest_part_3(data);
        assert_eq!(result, 30);
    }
}
