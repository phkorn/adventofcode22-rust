use std::io::Result;
use adventofcode22::read_lines_by_line;

const ROCK: isize = 1;
const PAPER:  isize = 2;
const SCISSOR:  isize = 3;
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
    enemy: isize,
    player: isize,
}

fn to_cryptic_player_move(txt: &str) -> Result<CrypticPlayerMove> {
    match txt {
        "X" => Ok(CrypticPlayerMove::X),
        "Y" => Ok(CrypticPlayerMove::Y),
        "Z" => Ok(CrypticPlayerMove::Z),
        _ => panic!("unhandled cryptic player move")
    }
}

fn to_enemy_move(txt: &str) -> Result<EnemyMove> {
    match txt {
        "A" => Ok(EnemyMove::A),
        "B" => Ok(EnemyMove::B),
        "C" => Ok(EnemyMove::C),
        _ => panic!("unhandled enemy move")
    }
}

fn to_result(m: Move) -> Result<RPSResult> {
    match m {
        Move {enemy: ROCK, player: ROCK } => Ok(RPSResult::Draw),
        Move {enemy: ROCK, player: PAPER } => Ok(RPSResult::Won),
        Move {enemy: ROCK, player: SCISSOR } => Ok(RPSResult::Lost),
        Move {enemy: PAPER, player: PAPER } => Ok(RPSResult::Draw),
        Move {enemy: PAPER, player: SCISSOR } => Ok(RPSResult::Won),
        Move {enemy: PAPER, player: ROCK } => Ok(RPSResult::Lost),
        Move {enemy: SCISSOR, player: SCISSOR } => Ok(RPSResult::Draw),
        Move {enemy: SCISSOR, player: ROCK } => Ok(RPSResult::Won),
        Move {enemy: SCISSOR, player: PAPER } => Ok(RPSResult::Lost),
        _ => panic!("unhandled move to result")
    }
}
fn main() ->  Result<()> {
    let path = "src/bin/02/input.txt";
    let mut sum = 0;

    read_lines_by_line(path, |line|{
        let split: Vec<&str> = line.split_whitespace().collect();

        let enemy_move_letter = to_enemy_move(split.first().unwrap())?;
        let enemy_move = enemy_move_letter as isize;

        let cryptic_player_move_letter = to_cryptic_player_move(split.last().unwrap())?;
        let cryptic_player_move = cryptic_player_move_letter as isize;

        let m: Move = Move { enemy:enemy_move, player: cryptic_player_move };

        let result = to_result(m)?;
        let result_game = result as isize;
        let result_round = result_game + cryptic_player_move;

        println!("enemyL {:?} enemyV {:?}, playerL {:?} playerV {:?}, result {:?} resultG {:?} resultR {:?} sum {}", enemy_move_letter,enemy_move, cryptic_player_move_letter, cryptic_player_move, result, result_game, result_round, sum);
        sum += result_round;
        Ok(())
    })?;

    println!("\nsum: {}", sum);

    Ok(())
}
