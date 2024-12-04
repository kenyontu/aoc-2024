use regex::Regex;

pub fn solve(input: &str) -> u32 {
    let re =
        Regex::new(r"((?<instruction>do(n't|)\(\))|(?<mul>mul\((\d{1,3}),(\d{1,3})\)))").unwrap();
    let mut is_enabled = true;
    let mut sum = 0;
    re.captures_iter(input).for_each(|c| {
        if let Some(instruction) = c.name("instruction") {
            is_enabled = match instruction.as_str() {
                "do()" => true,
                _ => false,
            };
        }

        if !is_enabled {
            return;
        }

        if let Some(_) = c.name("mul") {
            let num1 = c.get(5).unwrap().as_str().parse::<u32>().unwrap();
            let num2 = c.get(6).unwrap().as_str().parse::<u32>().unwrap();
            sum += num1 * num2;
        }
    });

    sum
}
