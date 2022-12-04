use std::collections::HashMap;

fn main() {
    let mut input = include_str!("input.txt").lines();
    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();
    let draws:Vec<i32> = input.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>(); // just panic
    input.next(); // skip first empty line
    boards.push(Vec::new());
    let mut curr_board = 0;
    let mut curr_row = 0;
    for line in input {
        if line.eq("") {
            // Reset to a new board
            let board: Vec<Vec<i32>> = Vec::new();
            curr_board = curr_board + 1;
            curr_row = 0;
            boards.push(board);
        } else {
            boards[curr_board].push(line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>().clone());
            curr_row = curr_row + 1;
        }
    }
    let mut winners: Vec<bool> = vec![false; boards.len()];
    let mut last_win = 0;
    let mut last_draw = 0;
    for draw in draws {
        for i in 0..boards.len()  {
            if !winners[i] && mark_board(draw, &mut boards[i]) {
                if check_winner(boards[i].clone()) {
                    last_win = i;
                    last_draw = draw;
                    winners[i] = true;
                }
            }
        }
    }
    println!("Last winner {} with sum: {}, draw: {}", last_win + 1,sum_unmarked(boards[last_win].clone()), last_draw);
}

fn print_boards(boards: Vec<Vec<Vec<i32>>>) {
    for board in boards {
        print_board(board);
        println!("");
    }
}

fn print_board(board: Vec<Vec<i32>>) {
    for row in board {
        for col in row {
            print!("{}\t", col);
        }
        println!("");
    }
}
fn sum_unmarked(board: Vec<Vec<i32>>) -> i32 {
    let mut sum:i32 = 0;
    for row in board {
        for val in row {
            if val != -1 {
                sum = sum + val;
            }
        }
    }
    return sum;
}

fn check_winner(board: Vec<Vec<i32>>) -> bool {
    let mut in_a_row = 0;
    let mut in_a_col = 0;
    // let mut left_diag = 0;
    // let mut right_diag = 0;
    // // Diaganol wins 
    // for i in 0..board.len() {
    //     left_diag = left_diag + board[i][i];
    //     right_diag = right_diag + board[board.len()-i-1][board.len()-i-1];
    // }
    // if left_diag == -5 || right_diag == -5 {
    //     return true;
    // }
    // Row and column wins
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            in_a_row = in_a_row + board[i][j];
            in_a_col = in_a_col + board[j][i];
        }
        if in_a_row == -5 || in_a_col == -5 {
            return true;
        }
        in_a_row = 0;
        in_a_col = 0;
    }
    return false;
}

// Returns true if found in board
fn mark_board(value: i32, board: &mut Vec<Vec<i32>>) -> bool {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == value {
                board[i][j] = -1;
                return true;
            }
        }
    }
    return false;
}