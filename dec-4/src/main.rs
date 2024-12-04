fn count_word_occurrences(grid: &[Vec<char>]) -> usize {
    const DIRECTIONS: [(isize, isize); 8] = [
        (0, 1),   // horizontal right
        (1, 0),   // vertical down
        (1, 1),   // diagonal down right
        (1, -1),  // diagonal down left
        (0, -1),  // horizontal left
        (-1, 0),  // vertical up
        (-1, -1), // diagonal up left
        (-1, 1),  // diagonal up right
    ];

    let rows = grid.len();
    let cols = grid[0].len();

    let word_chars: Vec<char> = "XMAS".chars().collect();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &DIRECTIONS {
                let mut matched = true;
                for i in 0..4 {
                    let nr = r as isize + i as isize * dr;
                    let nc = c as isize + i as isize * dc;

                    // oob or ne
                    if nr < 0
                        || nr >= rows as isize
                        || nc < 0
                        || nc >= cols as isize
                        || grid[nr as usize][nc as usize] != word_chars[i]
                    {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_shape_occurrences(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if is_xmas(grid, r, c) {
                count += 1;
            }
        }
    }

    count
}

fn is_xmas(grid: &[Vec<char>], r: usize, c: usize) -> bool {
    // char with | is 'pointed' to in the matrix search
    // S.S
    // | |
    // .A.
    //  |
    // M.M
    // | |
    let top_left = matches_mas(grid[r - 1][c - 1], grid[r][c], grid[r + 1][c + 1]);
    let top_right = matches_mas(grid[r - 1][c + 1], grid[r][c], grid[r + 1][c - 1]);
    top_left && top_right
}

fn matches_mas(m: char, a: char, s: char) -> bool {
    a == 'A' && ((m == 'M' && s == 'S') || (m == 'S' && s == 'M'))
}

fn main() {
    let args = common::args();
    let buf = args.file();
    let as_shape = args.boolean_flag("--shape");

    let grid: Vec<Vec<char>> = buf.lines().map(|line| line.chars().collect()).collect();

    let occurrences = if as_shape {
        count_shape_occurrences(&grid)
    } else {
        count_word_occurrences(&grid)
    };

    println!("occurrences: {}", occurrences);
}
