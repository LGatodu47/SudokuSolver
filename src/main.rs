use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

// Three libraries are used:
// - clap for program argument parsing
// - rand for random number generation
// - regex for regex matching in input strings
use clap::{arg, Arg, Command, value_parser};
use rand::{Rng, thread_rng};
use regex::Regex;

#[cfg(test)]
mod tests;

/// Structure that represents a Sudoku grid (9*9)
struct SudokuGrid {
    /// size must be 81
    data: Vec<u8>
}

impl SudokuGrid {
    fn set(&mut self, x:usize, y:usize, value: u8) {
        self.data[y * 9 + x] = value
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        match self.data.get(y * 9 + x) {
            Some(&num) => num,
            None => 0
        }
    }

    /// Returns a vec of all the values in the specified row of the grid.
    fn row(&self, y: usize) -> Vec<u8> {
        let mut row_contents = Vec::with_capacity(9);

        for x in 0..9 {
            let value = self.get(x, y);
            row_contents.push(value);
        }

        row_contents
    }

    /// Returns a vec of all the values in the specified column of the grid.
    fn column(&self, x: usize) -> Vec<u8> {
        let mut column_contents = Vec::with_capacity(9);

        for y in 0..9 {
            let value = self.get(x, y);
            column_contents.push(value)
        }

        column_contents
    }

    /// Returns a vec of all the values in the specified group (3*3 cell) of the grid.
    fn group(&self, x: usize, y:usize) -> Vec<u8> {
        let mut group_contents = Vec::with_capacity(9);

        let group_start_x = x - x % 3;
        let group_start_y = y - y % 3;

        for y_offset in 0..3 {
            for x_offset in 0..3 {
                let value = self.get(group_start_x + x_offset, group_start_y + y_offset);
                group_contents.push(value)
            }
        }

        group_contents
    }

    /// Checks whether the given value can be inserted in the given location (assuming there is no value already).
    /// This check is done according to the sudoku rules:
    /// - All digits on the row must be unique
    /// - All digits on the column must be unique
    /// - All digits in the 3x3 group must be unique
    fn check(&self, x: usize, y: usize, value: u8) -> bool {
        if self.row(y).contains(&value) {
            false
        } else if self.column(x).contains(&value) {
            false
        } else if self.group(x, y).contains(&value) {
            false
        } else {
            true
        }
    }

    /// Checks if the grid can be solved or not.
    fn check_grid(&self) -> bool {
        if self.is_empty() {
            return false
        }

        for y in 0..8 {
            for x in 0..8 {
                let value = self.get(x, y);
                if value != 0 {
                    // We filter and count occurrences because in opposition to `check()` the value we check for is already present.
                    if self.row(y).iter().filter(|&&v| v == value).count() > 1 {
                        return false
                    } else if self.column(x).iter().filter(|&&v| v == value).count() > 1 {
                        return false
                    } else if self.group(x, y).iter().filter(|&&v| v == value).count() > 1 {
                        return false
                    }
                }
            }
        }

        true
    }

    /// Returns true if there is no value set in the grid.
    fn is_empty(&self) -> bool {
        !self.data.iter().any(|&v| v > 0)
    }

    /// Creates an empty grid
    fn empty() -> SudokuGrid {
        SudokuGrid {
            data: vec![0; 81]
        }
    }

    /// Creates a grid with random values.
    /// The returned grid may not be a valid sudoku grid.
    fn randomly_filled() -> SudokuGrid {
        let mut data: Vec<u8> = vec![0; 81];

        let mut rng = thread_rng();

        for i in 0..(9*9) {
            if rng.gen_range(0..5) == 0 {
                data[i] = rng.gen_range(1..=9)
            }
        }

        SudokuGrid {
            data
        }
    }

    /// Creates a valid sudoku grid with random values.
    /// The valid grid is obtained after multiple iterations of `randomly_filled()`, therefore this method might return an empty grid.
    fn valid_random() -> SudokuGrid {
        let mut i = 0;
        while i < 10000 {
            let random_grid = SudokuGrid::randomly_filled();
            if random_grid.check_grid() {
                return random_grid
            }
            i += 1
        }

        SudokuGrid::empty()
    }

    /// Creates a grid with values from an example sudoku.
    fn example_grid() -> SudokuGrid {
        SudokuGrid {
            data: vec![
                5, 3, 0,   0, 7, 0,   0, 0, 0,
                6, 0, 0,   1, 9, 5,   0, 0, 0,
                0, 9, 8,   0, 0, 0,   0, 6, 0,

                8, 0, 0,   0, 6, 0,   0, 0, 3,
                4, 0, 0,   8, 0, 3,   0, 0, 1,
                7, 0, 0,   0, 2, 0,   0, 0, 6,

                0, 6, 0,   0, 0, 0,   2, 8, 0,
                0, 0, 0,   4, 1, 9,   0, 0, 5,
                0, 0, 0,   0, 8, 0,   0, 7, 9
            ]
        }
    }

    /// Creates a grid holding the specified data.
    fn from_data(data: &[u8]) -> SudokuGrid {
        SudokuGrid {
            data: Vec::from(data)
        }
    }
}

// Display implementation for SudokuGrid: helps with displaying the grid in the console.
impl Display for SudokuGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("\n");
        s.push_str("|-----------------|\n");

        for row_index in 0..9 {
            s.push_str("| ");

            for cell_index in 0..9 {
                let num = self.data.get(row_index * 9 + cell_index).filter(|&&v| v != 0).map(|v| v.to_string()).unwrap_or("_".to_string());

                if cell_index != 0 && cell_index % 3 == 0 {
                    s.push_str(" | ")
                }

                s.push_str(&num);
            }
            s.push_str(" |");
            s.push('\n');

            if (row_index + 1) % 3 == 0 {
                s.push_str("|-----------------|\n")
            }
        }

        f.write_str(&s)
    }
}

// Clone implementation for SudokuGrid: helps with making a copy of an existing grid.
impl Clone for SudokuGrid {
    fn clone(&self) -> Self {
        SudokuGrid {
            data: self.data.clone()
        }
    }
}

/// Enum of the error kinds that the process of solving can encounter.
enum SudokuSolvingError {
    InvalidGrid,
    Unsolvable,
    IterationCountOverflow
}

// Display implementation for SudokuSolvingError: helps with displaying the error after it has been caught.
impl Display for SudokuSolvingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SudokuSolvingError::InvalidGrid => f.write_str("The supplied sudoku grid is invalid!"),
            SudokuSolvingError::Unsolvable => f.write_str("The supplied sudoku is unsolvable!"),
            SudokuSolvingError::IterationCountOverflow => f.write_str("The solving process was abnormally long and therefore interrupted.")
        }
    }
}

/// Function that solves a sudoku grid.
/// It takes two parameters: the grid to solve and the maximum amount of iterations it can take to solve
fn solve(grid: SudokuGrid, max_iterations: u32) -> Result<SudokuGrid, SudokuSolvingError> {
    if !grid.check_grid() {
        return Err(SudokuSolvingError::InvalidGrid)
    }

    let mut solved_grid = grid.clone();

    // Keep track of the number of iterations
    let mut iteration_count: u32 = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;
    // If we're iterating backward, it means we encountered a dead end with the current combination. We therefore go back to change it and try with another combination.
    let mut iterating_forward = true;

    while iteration_count < max_iterations {
        // Check that we're not trying to replace a preset digit
        if grid.get(x, y) == 0 {
            if iterating_forward {
                // Whether a digit can satisfy the cell at the current pos or not
                let mut invalid = true;
                for value in 1..=9 {
                    if solved_grid.check(x, y, value) {
                        invalid = false;
                        solved_grid.set(x, y, value);
                        break
                    }
                }

                if invalid {
                    // no digit could satisfy the cell we are trying to fill, so we need to go back and change the previous cells.
                    iterating_forward = false;
                    // Common block to go back. If we try going back while x = 0 and y = 0, the sudoku must be unsolvable
                    if x == 0 {
                        if y > 0 {
                            x = 8;
                            y -= 1;
                        } else {
                            return Err(SudokuSolvingError::Unsolvable)
                        }
                    } else {
                        x -= 1
                    }
                } else {
                    // Common block to go forward: we break off the loop when we complete the last index.
                    if x >= 8 {
                        if y >= 8 {
                            break;
                        } else {
                            x = 0;
                            y += 1;
                        }
                    } else {
                        x += 1;
                    }
                }
            }
            else { // We're currently in the case where we got to a dead end earlier and we're trying to go back and change the previous digits
                // Digit that the cell currently holds
                let current_value = solved_grid.get(x, y);

                let mut invalid = true;
                // Iterate through all the digits, if we can't satisfy the conditions we need to go back even further.
                for value in current_value..=9 {
                    if solved_grid.check(x, y, value) {
                        invalid = false;
                        solved_grid.set(x, y, value);
                        break
                    }
                }

                if invalid {
                    // We go back again so we reset this value to its original state
                    solved_grid.set(x, y, 0);
                    // Common block: back
                    if x == 0 {
                        if y > 0 {
                            x = 8;
                            y -= 1;
                        } else {
                            return Err(SudokuSolvingError::Unsolvable)
                        }
                    } else {
                        x -= 1
                    }
                } else {
                    iterating_forward = true;

                    // Common block: forward
                    if x >= 8 {
                        if y >= 8 {
                            break;
                        } else {
                            x = 0;
                            y += 1;
                        }
                    } else {
                        x += 1;
                    }
                }
            }
        } else { // There is a preset digit at the current position, we continue forward or go back depending on the direction we were going before.
            if iterating_forward {
                // Common block: forward
                if x >= 8 {
                    if y >= 8 {
                        break;
                    } else {
                        x = 0;
                        y += 1;
                    }
                } else {
                    x += 1;
                }
            } else {
                // Common block: back
                if x == 0 {
                    if y > 0 {
                        x = 8;
                        y -= 1;
                    } else {
                        return Err(SudokuSolvingError::Unsolvable)
                    }
                } else {
                    x -= 1
                }
            }
        }

        iteration_count += 1;
    }

    // The sudoku couldn't be solved because it probably got into an infinite loop somewhere
    if iteration_count == max_iterations {
        return Err(SudokuSolvingError::IterationCountOverflow)
    }

    Ok(solved_grid)
}

const MAX_ITERATIONS_DEFAULT: u32 = 1000000;

/// Parses the program arguments using clap into a Result that either holds a tuple of our two arguments or a String describing an error.
/// TODO: Better error handling/description.
fn parse_arguments() -> Result<(SudokuGrid, u32), String> {
    let matches = Command::new("SudokuSolver")
        .about("Solves Sudoku puzzles!")
        .arg(
            arg!(--templates "Lists all the available sudoku grid templates.")
                .required(false)
        )
        .arg(
            Arg::new("grid")
                .short('g')
                .long("grid")
                .value_name("TEMPLATE | DATA | FILE")
                .help("Name of template, direct or file data (numbers separated by commas) of the sudoku grid to solve.")
                .required_unless_present("templates")
        )
        .arg(
            arg!(--max_solving_iterations <MAX_ITERATIONS> "Maximum number of iterations before the solving process gives up (default is 1000000).")
                .required(false)
                .value_parser(value_parser!(u32).range(1..))
        ).get_matches();

    // Print the available templates
    if matches.get_flag("templates") {
        println!("Here are the available templates:");
        println!("'example': a hard-coded example sudoku grid.");
        println!("'random': a randomly generated valid grid.");

        return Err(String::new())
    }

    let grid = matches.get_one::<String>("grid").map(|info| {
        // We first check for templates
        match info.as_str() {
            "example" => Some(SudokuGrid::example_grid()),
            "random" => Some(SudokuGrid::valid_random()),
            _ => {
                // Then for row data
                let data = Regex::new(r"(\d,?)+")
                    .ok()// We're only interested into the regex
                    .map(|regex| regex.find(info))// We obtain the part we want
                    .flatten()// We flatten the option
                    .map(|m| m.as_str().to_string())// We convert the match into an &str
                    .or(read_data_from_file(info))// If there is no match, meaning a path might have been specified, we try reading the file.
                    .map(|s| {
                        // We split the resulting part
                        let digits = s.split(',').collect::<Vec<&str>>();
                        // We ensure that the content is of the right size
                        if digits.len() != 81 {
                            return None
                        }
                        // We map all the values in the vec from &str to u8
                        Some(digits.iter().map(|s| s.parse().unwrap_or(0)).collect::<Vec<u8>>())
                    }).flatten();

                data.map(|v| SudokuGrid::from_data(&v))
            }
        }
    }).flatten().ok_or(String::from("grid info couldn't be parsed. Try using a template or directly specifying the grid data (with numbers between commas, like so: '0,6,4,8,0,0,1,0,...')."))?;

    Ok((grid, matches.get_one::<u32>("max_solving_iterations").map(|&r| r).unwrap_or(MAX_ITERATIONS_DEFAULT)))
}

/// Reads the content of a file at the path referred by a String.
fn read_data_from_file(path: &String) -> Option<String> {
    File::open(path)
        .ok()// We don't care about the error
        .map(|mut file| {
            let mut content = String::new();
            file.read_to_string(&mut content).ok();
            content
        })// Maps the file to its actual content
        .map(|s| s.trim().replace(' ', "")) // Trims the content string and gets rid of useless whitespaces.
}

fn main() {
    match parse_arguments() {
        Ok((grid, max_iterations)) => {
            println!("String representation of the grid: {}", grid);
            println!("Lets try to solve this sudoku...");
            match solve(grid, max_iterations) {
                Ok(solved_grid) => println!("Solved the given grid! Here it is: {}", solved_grid),
                Err(err) => println!("Failed to solve the sudoku: {}", err)
            }
        },
        Err(err) => {
            // empty error means no error
            if !err.is_empty() {
                println!("Invalid arguments: {}", err)
            }
        }
    }
}