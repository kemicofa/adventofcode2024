pub fn solve(raw_input: &str) -> u64 {
    let mut list1: Vec<u64> = Vec::new();
    let mut list2: Vec<u64> = Vec::new();

    for line in raw_input.trim().split('\n') {
        let (part1, part2) = line.trim().split_once("   ").unwrap();
        list1.push(part1.parse().unwrap());
        list2.push(part2.parse().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut total_distance = 0;

    for i in 0..list1.len() {
        total_distance += list1[i].abs_diff(list2[i]);
    }
    total_distance
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
        assert_eq!(result, 1590491);
    }
}
