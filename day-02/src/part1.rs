pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report| {
            let (min, max) = if report[0] < report[1] {
                (1, 3)
            } else {
                (-3, -1)
            };

            for i in 1..report.len() {
                if report[i] < report[i - 1] + min || report[i] > report[i - 1] + max {
                    return false;
                }
            }
            return true;
        })
        .count()
}
