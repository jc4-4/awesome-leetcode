// https://leetcode.com/problems/word-search

// Potential optimizations:
// 1. check character frequency for feasibility
// 2. search from the end
// 3. split the search at (x, y) => need to make sure path not crossed

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let rows = board.len();
    let cols = board[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    for x in 0..rows {
        for y in 0..cols {
            if backtrack(x as i32, y as i32, 0, &mut visited, &board, &word) {
                return true;
            }
        }
    }
    false
}

fn backtrack(
    x: i32,
    y: i32,
    step: usize,
    path: &mut Vec<Vec<bool>>,
    board: &Vec<Vec<char>>,
    word: &String,
) -> bool {
    // out of bound
    if x < 0 || x >= board.len() as i32 || y < 0 || y >= board[0].len() as i32 {
        return false;
    }

    // do not repeat path
    if path[x as usize][y as usize] {
        return false;
    }

    // character does not match
    if board[x as usize][y as usize] != word.chars().nth(step).expect("no char!") {
        return false;
    }

    path[x as usize][y as usize] = true;
    let found = step + 1 == word.len()
        || backtrack(x - 1, y, step + 1, path, board, word)
        || backtrack(x + 1, y, step + 1, path, board, word)
        || backtrack(x, y - 1, step + 1, path, board, word)
        || backtrack(x, y + 1, step + 1, path, board, word);
    path[x as usize][y as usize] = false;
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(board: &[&[char]]) -> Vec<Vec<char>> {
        board.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn test_sample1() {
        assert!(exist(
            to_vec(&[
                &['A', 'B', 'C', 'E'],
                &['S', 'F', 'C', 'S'],
                &['A', 'D', 'E', 'E']
            ]),
            "ABCCED".to_string()
        ))
    }

    #[test]
    fn test_sample2() {
        assert!(exist(
            to_vec(&[
                &['A', 'B', 'C', 'E'],
                &['S', 'F', 'C', 'S'],
                &['A', 'D', 'E', 'E']
            ]),
            "SEE".to_string()
        ))
    }

    #[test]
    fn test_sample3() {
        assert!(!exist(
            to_vec(&[
                &['A', 'B', 'C', 'E'],
                &['S', 'F', 'C', 'S'],
                &['A', 'D', 'E', 'E']
            ]),
            "ABCB".to_string()
        ))
    }

    #[test]
    fn test_wrong_answer1() {
        assert!(exist(to_vec(&[&['A']]), "A".to_string()))
    }

    #[test]
    fn test_wrong_answer2() {
        assert!(exist(
            to_vec(&[&['C', 'A', 'A'], &['A', 'A', 'A'], &['B', 'C', 'D']]),
            "AAB".to_string()
        ))
    }
}
