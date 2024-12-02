use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    let mut left_list = Vec::new();
    let mut map = HashMap::new();

    input.lines().for_each(|l| {
        let mut split = l.split("   ");

        left_list.push(split.next().unwrap().parse::<u32>().unwrap());

        let num = split.next().unwrap().parse::<u32>().unwrap();
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    });

    left_list.sort();

    let mut sum = 0;

    for num in left_list.iter() {
        if let Some(count) = map.get(num) {
            sum += num * count;
        }
    }

    sum
}
