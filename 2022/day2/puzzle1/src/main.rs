fn main() {
    let inputtext = include_str!("input.txt");
    let mut total_points:i32 = 0;
    for line in inputtext.lines() {
        let moves = line.split(" ").collect::<Vec<&str>>();
        let round_pts = determine_winner(moves.get(0).unwrap(), moves.get(1).unwrap());
        total_points += round_pts;
    }
    println!("Total points: {}", total_points);
}

fn determine_winner(player1: &str, player2: &str) -> i32 {
    const DRAW_PTS: i32 = 3;
    const LOSE_PTS: i32 = 0;
    const WIN_PTS: i32 = 6;
    const ROCK_PTS: i32 = 1;
    const PAPER_PTS: i32 = 2;
    const SCISSOR_PTS: i32 = 3;

    const M_ROCK: &str = "X";
    const M_PAPER: &str = "Y";
    const M_SCISSOR: &str = "Z";

    const T_ROCK: &str = "A";
    const T_PAPER: &str = "B";
    const T_SCISSOR: &str = "C";
    let tup = (player1,player2);
    let result = match tup {
        (T_ROCK, M_ROCK) => ROCK_PTS+DRAW_PTS,
        (T_ROCK, M_PAPER) => PAPER_PTS+WIN_PTS,
        (T_ROCK, M_SCISSOR) => SCISSOR_PTS+LOSE_PTS,
        (T_PAPER, M_ROCK) => ROCK_PTS+LOSE_PTS,
        (T_PAPER, M_PAPER) => PAPER_PTS+DRAW_PTS,
        (T_PAPER, M_SCISSOR) => SCISSOR_PTS+WIN_PTS,
        (T_SCISSOR, M_ROCK) => ROCK_PTS+WIN_PTS,
        (T_SCISSOR, M_PAPER) => PAPER_PTS+LOSE_PTS,
        (T_SCISSOR, M_SCISSOR) => SCISSOR_PTS+DRAW_PTS,
        _ => panic!("Unsupported value found"),
    };
    return result;
}