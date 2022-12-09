const DRAW_PTS: i32 = 3;
const LOSE_PTS: i32 = 0;
const WIN_PTS: i32 = 6;
const ROCK_PTS: i32 = 1;
const PAPER_PTS: i32 = 2;
const SCISSOR_PTS: i32 = 3;

const SHOULD_LOSE: &str = "X";
const SHOULD_DRAW: &str = "Y";
const SHOULD_WIN: &str = "Z";

const ROCK: &str = "A";
const PAPER: &str = "B";
const SCISSOR: &str = "C";

fn main() {
    let inputtext = include_str!("input.txt");
    let mut total_points:i32 = 0;
    for line in inputtext.lines() {
        let moves = line.split(" ").collect::<Vec<&str>>();
        let opponent_move = moves.get(0).unwrap();
        let outcome = moves.get(1).unwrap();
        let my_move = match (*opponent_move, *outcome) {
            (ROCK, SHOULD_LOSE) => SCISSOR,
            (ROCK, SHOULD_DRAW) => ROCK,
            (ROCK, SHOULD_WIN) => PAPER,
            (PAPER, SHOULD_LOSE) => ROCK,
            (PAPER, SHOULD_DRAW) => PAPER,
            (PAPER, SHOULD_WIN) => SCISSOR,
            (SCISSOR, SHOULD_LOSE) => PAPER,
            (SCISSOR, SHOULD_DRAW) => SCISSOR,
            (SCISSOR, SHOULD_WIN) => ROCK,
            _ => panic!("Unsupported value found"),
        };
        let round_pts =  determine_winner(opponent_move, my_move);
        total_points += round_pts;
    }
    println!("Total points: {}", total_points);
}

fn determine_winner(player1: &str, player2: &str) -> i32 {
    let result = match (player1,player2) {
        (ROCK, ROCK) => ROCK_PTS+DRAW_PTS,
        (ROCK, PAPER) => PAPER_PTS+WIN_PTS,
        (ROCK, SCISSOR) => SCISSOR_PTS+LOSE_PTS,
        (PAPER, ROCK) => ROCK_PTS+LOSE_PTS,
        (PAPER, PAPER) => PAPER_PTS+DRAW_PTS,
        (PAPER, SCISSOR) => SCISSOR_PTS+WIN_PTS,
        (SCISSOR, ROCK) => ROCK_PTS+WIN_PTS,
        (SCISSOR, PAPER) => PAPER_PTS+LOSE_PTS,
        (SCISSOR, SCISSOR) => SCISSOR_PTS+DRAW_PTS,
        _ => panic!("Unsupported value found"),
    };
    return result;
}