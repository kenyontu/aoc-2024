use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub fn solve(input: &str) -> u32 {
    let mut lines = input.lines().into_iter();

    // Map to group a number and the numbers it needs to be less than
    //
    // Example: [ 1, 2, 3, 4, 5, 6, 7, 8 ]
    //
    // { 3: [4, 5], 1; [2, 3] }
    // - 3 needs to be less than 4 and 5
    // - 1 needs to be less than 2 and 3
    let mut lower_than_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    loop {
        match lines.next() {
            Some(line) => {
                if line.is_empty() {
                    break;
                }

                let split = line.split('|').collect::<Vec<_>>();
                lower_than_map
                    .entry(split[0].parse().unwrap())
                    .and_modify(|entry| {
                        entry.insert(split[1].parse().unwrap());
                    })
                    .or_insert(HashSet::from([split[1].parse().unwrap()]));
            }
            _ => break,
        }
    }

    let mut sum = 0;

    for line in lines {
        let mut seen: HashSet<u32> = HashSet::new();

        let numbers = line
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut is_valid = true;
        for i in 0..numbers.len() {
            let n = numbers[i] as u32;
            if let Some(lower_than) = lower_than_map.get(&n) {
                if seen.intersection(lower_than).count() > 0 {
                    is_valid = false;
                    break;
                }
            };

            seen.insert(n);
        }

        if !is_valid {
            let mut numbers = numbers;

            numbers.sort_by(|a, b| {
                if let Some(lower_than) = lower_than_map.get(a) {
                    if lower_than.get(b).is_some() {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            });

            let index = numbers.len() / 2;
            sum += numbers[index];
        }
    }

    sum
}
