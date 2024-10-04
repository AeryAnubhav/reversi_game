// Assignmnet 1
// Student Name: Anubhav Aery
// Student Number: 1005839513
use std::cmp::Ordering;
use std::io::{self, Write};

// Function to print the current state of the board
fn print_board(board: &[Vec<char>]) {
    // Column Headers (a-h)
    println!("  abcdefgh");

    // Print row headers along with the board rows
    for (i, row) in board.iter().enumerate() {
        // Print row labels (a-h)
        print!("{} ", (b'a' + i as u8) as char);
        // Print each column element for current row
        for col in row {
            print!("{}", col); // Print each column element ('.','B' or 'W')
        }
        println!(); // Move to the next line after printing each row
    }
}

// Function to get the player's move input
fn get_player_move(current_player: char) -> String {
    // Ask player for move
    print!("Enter move for colour {} (RowCol): ", current_player);
    io::stdout().flush().unwrap(); //Ensuring prompt is printed before taking input

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input"); // Capture player's input
    input.trim().to_string() // Return the input without leading whitespace
}

// Function to parse the player's move (Eg: converts 'cd' to (2,3))
fn parse_move(input: &str) -> Option<(usize, usize)> {
    // Making sure length of string is exactly two which corresponds to row and column
    if input.len() != 2 {
        return None; // Invalid input if not exactly 2 characters
    }

    // Convert row and column from letter to indices
    let mut chars = input.chars();
    let row_char = chars.next()?;
    let col_char = chars.next()?;

    // Making sure characters are in range 'a' to 'h'
    if !('a'..='h').contains(&row_char) || !('a'..='h').contains(&col_char) {
        return None;
    }

    // Convert row and column from 'a' - 'h' to 0-7 indices
    let row = row_char as usize - 'a' as usize;
    let col = col_char as usize - 'a' as usize;

    Some((row, col)) // Return the parsed move as a tuple (row,col)
}

// Function to place a piece on the board if the spot is empty
fn place_piece(board: &mut [Vec<char>], player: char, row: usize, col: usize) -> bool {
    // Check if valid empty move (not occupied)
    if board[row][col] == '.' {
        board[row][col] = player; // Place the player's piece on the board
        true // Move was successful
    } else {
        false // Move was invalid (already occupied)
    }
}

// Helper function to check and flip opponent's pieces in a specific direction
fn check_and_flip_direction(
    board: &mut [Vec<char>],
    player: char,
    row: usize,
    col: usize,
    row_dir: isize,
    col_dir: isize,
) -> bool {
    let mut current_row = row as isize + row_dir;
    let mut current_col = col as isize + col_dir;
    let opponent = if player == 'B' { 'W' } else { 'B' };
    let mut to_flip = Vec::new();

    // Traverse in the specified direction
    while (0..8).contains(&current_row) && (0..8).contains(&current_col) {
        let current_piece = board[current_row as usize][current_col as usize];

        if current_piece == opponent {
            // Add opponent's piece to the flip list
            to_flip.push((current_row as usize, current_col as usize));
        } else if current_piece == player {
            // Flip all opponent's pieces before returning
            for (r, c) in &to_flip {
                board[*r][*c] = player;
            }
            return !to_flip.is_empty(); // Return true if any pieces were flipped
        } else {
            // If an empty spot was reached, flipping not possible
            return false;
        }

        current_row += row_dir;
        current_col += col_dir;
    }
    false // No pieces to flip in this direction
}

// Function to flip opponent's pieces after placing a piece on the board
fn flip_pieces(board: &mut [Vec<char>], player: char, row: usize, col: usize) -> bool {
    // All 8 possible directions for flipping pieces
    let directions = [
        (-1, 0),  // North
        (1, 0),   // South
        (0, -1),  // West
        (0, 1),   // East
        (-1, -1), // North-West
        (-1, 1),  // North-East
        (1, -1),  // South-West
        (1, 1),   // South-East
    ];

    let mut flipped = false;

    for &(row_dir, col_dir) in &directions {
        // Checking if flipped at least once
        if check_and_flip_direction(board, player, row, col, row_dir, col_dir) {
            flipped = true; // If pieces were flipped in any direction
        }
    }
    flipped
}

// Function to check if the player has any valid moves
fn player_has_valid_moves(board: &[Vec<char>], player: char) -> bool {
    // Iterate through the board and check each empty spot
    for (row_index, row) in board.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == '.' {
                // Temporarily place piece to check if it can flip opponent's pieces
                let mut temp_board = board.to_owned(); // Clone the board
                temp_board[row_index][col_index] = player; //Simulate placing the piece
                if flip_pieces(temp_board.as_mut_slice(), player, row_index, col_index) {
                    return true; // Valid move exists since able to place the pieces
                }
            }
        }
    }
    false // No valid moves left
}

// Function to calculate and return the score
fn calculate_score(board: &[Vec<char>]) -> (usize, usize) {
    let mut black_score = 0;
    let mut white_score = 0;

    // Count the number of black and white pieces on the board.
    for row in board.iter() {
        for &cell in row.iter() {
            match cell {
                'B' => black_score += 1, // Count black pieces
                'W' => white_score += 1, // Count white pieces
                _ => {}                  // Ignore empty spots
            }
        }
    }
    (black_score, white_score) // Return both scores
}

fn main() {
    // Initialize the board 8x8 board with dpts ('.') for empty spots
    let mut board = vec![vec!['.'; 8]; 8];

    // Initial 4 pieces in the center
    board[3][3] = 'W'; // White piece
    board[3][4] = 'B'; // Black piece
    board[4][3] = 'B'; // Black piece
    board[4][4] = 'W'; // White piece

    // Function to print the board (Function Call)
    print_board(&board);

    let mut current_player = 'B'; // Black always goes first
    let mut no_valid_moves_in_a_row = 0; // Initialize Counter

    loop {
        // Check if current player has valid moves
        if !player_has_valid_moves(&board, current_player) {
            println!("{} player has no valid move.", current_player);
            current_player = if current_player == 'B' { 'W' } else { 'B' }; // Switch players
            no_valid_moves_in_a_row += 1; // Count consecutive no-move turns
            if no_valid_moves_in_a_row == 2 {
                // Both players have no valid moves
                let (black_score, white_score) = calculate_score(&board);

                // Determine and print the winner
                match black_score.cmp(&white_score) {
                    Ordering::Greater => {
                        println!("Black wins by {} points!", black_score - white_score)
                    }
                    Ordering::Less => {
                        println!("White wins by {} points!", white_score - black_score)
                    }
                    Ordering::Equal => println!("Draw!"),
                }
                break;
            }
            continue; // Skip to the next iteration
        } else {
            no_valid_moves_in_a_row = 0; // Reset the counter if valid move exists
        }

        // Get player's current move (Function Call)
        let input = get_player_move(current_player);

        // Parse the player's move (Function Call)
        match parse_move(&input) {
            Some((row, col)) => {
                // Place piece on board (Function Call)
                if place_piece(&mut board, current_player, row, col) {
                    // Check if valid move (successfully flipped) (Function Call)
                    if flip_pieces(&mut board, current_player, row, col) {
                        print_board(&board); // Print the board after the move

                        // Switch player after each turn
                        current_player = if current_player == 'B' { 'W' } else { 'B' };
                    } else {
                        // Invalid move, undo the move
                        board[row][col] = '.';
                        println!("Invalid move. Try again.");
                        print_board(&board); // Print board again to show invalid move
                    }
                } else {
                    println!("Invalid move. Try again.");
                    print_board(&board); // Print board again for invalid input
                }
            }
            None => {
                println!("Invalid move. Try again."); // Print move again for invalid input
                print_board(&board);
            }
        }
    }
}
