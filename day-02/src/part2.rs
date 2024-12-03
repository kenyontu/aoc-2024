pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report| is_safe(report, 0))
        .count()
}

fn is_safe(report: &Vec<i32>, removed_count: usize) -> bool {
    let mut i = 1;
    let (min, max) = if report[i - 1] < report[i] {
        (1, 3)
    } else {
        (-3, -1)
    };

    while i < report.len() {
        if report[i] < report[i - 1] + min || report[i] > report[i - 1] + max {
            if removed_count > 0 {
                return false;
            }

            let a = {
                let (left, right) = report.split_at(i - 1);
                [left, &right[1..]].concat()
            };

            let b = {
                let (left, right) = report.split_at(i);
                [left, &right[1..]].concat()
            };

            // When the problem is found at position 2, there's a chance that removing
            // the first position makes the report safe
            if i == 2 {
                let c = report[1..].to_vec();
                return is_safe(&a, 1) || is_safe(&b, 1) || is_safe(&c, 1);
            } else {
                return is_safe(&a, 1) || is_safe(&b, 1);
            }
        }
        i += 1;
    }

    true
}
