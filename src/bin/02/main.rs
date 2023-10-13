use std::io::Result;
use adventofcode22::read_lines_by_line;

const ROCK: isize = 1;
const PAPER:  isize = 2;
const SCISSOR:  isize = 3;

#[derive(Debug, Clone, Copy)]
enum RPSMove {
    Rock = ROCK,
    Paper = PAPER,
    Scissor = SCISSOR,
}

#[derive(Debug, Clone, Copy)]
enum RPSResult {
    Lost = 0,
    Draw = 3,
    Won = 6
}

#[derive(Debug, Clone, Copy)]
enum CrypticPlayerMove {
    X = RPSMove::Rock as isize,
    Y = RPSMove::Paper as isize,
    Z = RPSMove::Scissor as isize,
}

#[derive(Debug, Clone, Copy)]
enum EnemyMove {
    A = RPSMove::Rock as isize,
    B = RPSMove::Paper as isize,
    C = RPSMove::Scissor as isize,
}


struct Move {
    enemy: RPSMove,
    player: RPSMove,
}

fn to_cryptic_player_move(txt: &str) -> CrypticPlayerMove {
    match txt {
        "X" => CrypticPlayerMove::X,
        "Y" => CrypticPlayerMove::Y,
        "Z" => CrypticPlayerMove::Z,
        _ => panic!("unhandled cryptic player move")
    }
}

fn to_enemy_move(txt: &str) -> EnemyMove {
    match txt {
        "A" => EnemyMove::A,
        "B" => EnemyMove::B,
        "C" => EnemyMove::C,
        _ => panic!("unhandled enemy move")
    }
}

fn to_suggested_result(txt: &str) -> RPSResult {
    match txt {
        "X" => RPSResult::Lost,
        "Y" =>RPSResult::Draw,
        "Z" => RPSResult::Won,
        _ => panic!("unhandled intended result")
    }
}

fn enemy_move_to_rps_move(enemy_move: &EnemyMove) -> RPSMove {
    match enemy_move {
        EnemyMove::A => RPSMove::Rock,
        EnemyMove::B => RPSMove::Paper,
        EnemyMove::C => RPSMove::Scissor
    }
}

fn cryptic_player_move_to_rps_move(enemy_move: &CrypticPlayerMove) -> RPSMove {
    match enemy_move {
        CrypticPlayerMove::X => RPSMove::Rock,
        CrypticPlayerMove::Y => RPSMove::Paper,
        CrypticPlayerMove::Z => RPSMove::Scissor
    }
}


fn suggested_rps_move(hint: (&RPSMove, &RPSResult)) -> RPSMove {
    match hint {
        (RPSMove::Rock, RPSResult::Won) => RPSMove::Paper,
        (RPSMove::Rock, RPSResult::Lost) => RPSMove::Scissor,
        (RPSMove::Rock, RPSResult::Draw) => RPSMove::Rock,
        (RPSMove::Paper, RPSResult::Won) => RPSMove::Scissor,
        (RPSMove::Paper, RPSResult::Lost) => RPSMove::Rock,
        (RPSMove::Paper, RPSResult::Draw) => RPSMove::Paper,
        (RPSMove::Scissor, RPSResult::Won) => RPSMove::Rock,
        (RPSMove::Scissor, RPSResult::Lost) => RPSMove::Paper,
        (RPSMove::Scissor, RPSResult::Draw) => RPSMove::Scissor,
    }
}

fn to_result(m: Move) -> RPSResult {
    match m {
        Move {enemy: RPSMove::Rock, player: RPSMove::Rock } => RPSResult::Draw,
        Move {enemy: RPSMove::Rock, player: RPSMove::Paper } => RPSResult::Won,
        Move {enemy: RPSMove::Rock, player: RPSMove::Scissor } => RPSResult::Lost,
        Move {enemy: RPSMove::Paper, player: RPSMove::Paper } => RPSResult::Draw,
        Move {enemy: RPSMove::Paper, player: RPSMove::Scissor } => RPSResult::Won,
        Move {enemy: RPSMove::Paper, player: RPSMove::Rock } => RPSResult::Lost,
        Move {enemy: RPSMove::Scissor, player: RPSMove::Scissor } => RPSResult::Draw,
        Move {enemy: RPSMove::Scissor, player: RPSMove::Rock } => RPSResult::Won,
        Move {enemy: RPSMove::Scissor, player: RPSMove::Paper } => RPSResult::Lost,
    }
}


fn first() -> Result<()> {
    let path = "src/bin/02/input.txt";
    let mut sum = 0;

    read_lines_by_line(path, |line|{
        let split: Vec<&str> = line.split_whitespace().collect();

        let enemy_move_letter = to_enemy_move(split.first().unwrap());
        let enemy_move = enemy_move_to_rps_move(&enemy_move_letter);

        let cryptic_player_move_letter = to_cryptic_player_move(split.last().unwrap());
        let player_move = cryptic_player_move_to_rps_move(&cryptic_player_move_letter);
        let player_move_num =  player_move as isize;

        // move is a keyword
        let mv: Move = Move { enemy:enemy_move, player: player_move };

        let result = to_result(mv);
        let result_game = result as isize;
        let result_round = result_game + player_move_num;

        println!("{:?} ⚔️ {:?} -> {:?} | #game {:?}, #round {:?}, sum {}", player_move, enemy_move, result, result_game, result_round, sum);
        sum += result_round;
        Ok(())
    })?;

    println!("\nTotal Result: {}", sum);

    Ok(())
} 

fn second() -> Result<()> {
    let path = "src/bin/02/input.txt";
    let mut sum = 0;

    read_lines_by_line(path, |line|{
        let split: Vec<&str> = line.split_whitespace().collect();

        let enemy_move_cryptic = to_enemy_move(split.first().unwrap());
        let enemy_move = enemy_move_to_rps_move(&enemy_move_cryptic);

        let intended_result_letter = to_suggested_result(split.last().unwrap());

        let player_move = suggested_rps_move((&enemy_move, &intended_result_letter));
        let player_move_num =  player_move as isize;
        
        let mv: Move = Move { enemy:enemy_move, player: player_move };

        let result = to_result(mv);
        let result_game = result as isize;
        let result_round = result_game + player_move_num;

        println!("{:?} ⚔️ {:?} -> {:?} | #game {:?}, #round {:?}, sum {}", player_move, enemy_move, result, result_game, result_round, sum);
        sum += result_round;
        Ok(())
    })?;

    println!("\nTotal Result: {}", sum);

    Ok(())
} 

fn main() ->  Result<()> {
    println!("##### first ####");
    // 12772
    first()?;

    println!("##### second ####");
    // 11618
    second()?;
    Ok(())
}
