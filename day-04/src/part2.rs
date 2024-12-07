const WORD: [char; 5] = ['A', 'M', 'S', 'M', 'S'];

pub fn solve(input: &str) -> usize {
    let puzzle = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // M S | M M | S M | S S
    //  A  |  A  |  A  |  A
    // M S | S S | S M | M M
    let offsets = vec![
        vec![
            (0, 0),   // center
            (-1, -1), // left-up
            (1, 1),   // right-down
            (-1, 1),  // left-down
            (1, -1),  // right-up
        ],
        vec![
            (0, 0),   // center
            (-1, -1), // left-up
            (1, 1),   // right-down
            (1, -1),  // right-up
            (-1, 1),  // left-down
        ],
        vec![
            (0, 0),   // center
            (1, -1),  // right-up
            (-1, 1),  // left-down
            (1, 1),   // right-down
            (-1, -1), // left-up
        ],
        vec![
            (0, 0),   // center
            (-1, 1),  // left-down
            (1, -1),  // right-up
            (1, 1),   // right-down
            (-1, -1), // left-up
        ],
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
    offsets: &[(isize, isize)],
    limits: &(usize, usize),
) -> bool {
    let mut i = 0;

    while i < WORD.len() {
        let row = start_pos.0 as isize + offsets[i].0;
        let col = start_pos.1 as isize + offsets[i].1;

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
