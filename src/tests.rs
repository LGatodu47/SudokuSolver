use super::*;

#[test]
fn solve_sudoku1() {
    let values = vec![0, 6, 0, 0, 0, 0, 9, 7, 0, 0, 3, 0, 8, 0, 4, 0, 0, 0, 2, 0, 0, 5, 9, 0, 0, 0, 0, 0, 7, 0, 0, 4, 0, 6, 0, 0, 0, 0, 5, 0, 0, 0, 1, 0, 0, 0, 0, 6, 0, 3, 0, 0, 8, 0, 0, 0, 0, 0, 5, 9, 0, 0, 1, 0, 0, 0, 1, 0, 7, 0, 3, 0, 0, 8, 1, 0, 0, 0, 0, 6, 0];
    let grid = SudokuGrid::from_data(values.as_slice());
    let solved = {
        match solve(grid, MAX_ITERATIONS_DEFAULT) {
            Ok(grid) => grid,
            Err(err) => panic!("Couldn't solve the test sudoku 1: {}", err)
        }
    };
    let expected = vec![8, 6, 4, 3, 1, 2, 9, 7, 5, 5, 3, 9, 8, 7, 4, 2, 1, 6, 2, 1, 7, 5, 9, 6, 3, 4, 8, 3, 7, 8, 9, 4, 1, 6, 5, 2, 4, 2, 5, 7, 6, 8, 1, 9, 3, 1, 9, 6, 2, 3, 5, 7, 8, 4, 7, 4, 3, 6, 5, 9, 8, 2, 1, 6, 5, 2, 1, 8, 7, 4, 3, 9, 9, 8, 1, 4, 2, 3, 5, 6, 7];
    assert_eq!(solved.data, expected, "Expected grid and solved grid contents didn't match.")
}