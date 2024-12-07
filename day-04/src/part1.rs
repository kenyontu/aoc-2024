const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

pub fn solve(input: &str) -> usize {
    let puzzle = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let offsets = vec![
        (0, -1),  // up
        (1, -1),  // right-up
        (1, 0),   // right
        (1, 1),   // right-down
        (0, 1),   // down
        (-1, 1),  // left-down
        (-1, 0),  // left
        (-1, -1), // left-up
    ];

    let mut found = 0;

    for i in 0..puzzle.len() {
        for j in 0..puzzle[i].len() {
            found += offsets
                .iter()
                .filter(|offset| {
                    search_xmas(&puzzle, (i, j), offset, &(puzzle.len(), puzzle[i].len()))
                })
                .count();
        }
    }

    return found;
}

fn search_xmas(
    puzzle: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    offset: &(isize, isize),
    limits: &(usize, usize),
) -> bool {
    let mut i = 0;

    while i < WORD.len() {
        let row = start_pos.0 as isize + offset.0 * i as isize;
        let col = start_pos.1 as isize + offset.1 * i as isize;

        if row < 0 || col < 0 || row >= limits.0 as isize || col >= limits.1 as isize {
            return false;
        }

        let row = row as usize;
        let col = col as usize;

        if puzzle[row][col] != WORD[i] {
            return false;
        }
        i += 1;
    }

    true
}
