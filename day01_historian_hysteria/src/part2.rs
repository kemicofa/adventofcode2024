use std::collections::HashMap;

pub fn solve(raw_input: &str) -> u64 {
    let mut list1: Vec<u64> = Vec::new();
    let mut list2: Vec<u64> = Vec::new();

    for line in raw_input.trim().split('\n') {
        let (part1, part2) = line.trim().split_once("   ").unwrap();
        list1.push(part1.parse().unwrap());
        list2.push(part2.parse().unwrap());
    }

    let mut map: HashMap<u64, u64> = HashMap::new();

    for value in list2 {
        if let Some(occurrence) = map.get_mut(&value) {
            *occurrence += 1;
        } else {
            map.insert(value, 1);
        }
    }

    let mut similarity_score = 0;

    for value in list1 {
       similarity_score += value * map.get(&value).unwrap_or(&0); 
    }

    similarity_score
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "#;
        let result = solve(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn it_works_with_the_puzzle() {
        let input = fs::read_to_string("./puzzle.txt").unwrap();
        let result = solve(input.as_str());
        assert_eq!(result, 22588371);
    }
}
