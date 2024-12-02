pub fn solve(input: &str) -> u32 {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    input.lines().for_each(|l| {
        let mut split = l.split("   ");

        left_list.push(split.next().unwrap().parse::<u32>().unwrap());
        right_list.push(split.next().unwrap().parse::<u32>().unwrap());
    });

    left_list.sort();
    right_list.sort();

    let mut sum = 0;

    for i in 0..left_list.len() {
        sum += left_list[i].abs_diff(right_list[i]);
    }

    sum
}
