use regex::Regex;

pub fn solve(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    for (_, [num1, num2]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap();
    }

    sum
}
