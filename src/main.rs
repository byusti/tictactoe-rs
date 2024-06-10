//Tic Tac Toe library
//Goals: make_move(), undo_move(), all_legal_moves()
// we will be using bitboards.
fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug, Clone)]
struct GameState {
    x_bitboard: Bitboard,
    o_bitboard: Bitboard,
    game_status: GameStatus,
    move_history: Vec<Move>,
    turn: Turn,
}

const NUMBER_OF_POSITIONS: u8 = 9;
const LAST_BIT_CHECKER: Bitboard = 0b000000001;

impl GameState {
    fn all_legal_moves(&self) -> Vec<Move> {
        match self.game_status {
            GameStatus::XWins => Vec::new(),
            GameStatus::OWins => Vec::new(),
            GameStatus::Draw => Vec::new(),
            GameStatus::InProgress => {
                let mut all_legal_moves = Vec::new();
                let mut all_occupied_bitboard = self.x_bitboard | self.o_bitboard;
                for n in (1..=NUMBER_OF_POSITIONS).rev() {
                    let result = all_occupied_bitboard & LAST_BIT_CHECKER;
                    match result {
                        LAST_BIT_CHECKER => (),
                        _ => {
                            let legal_move = Move::from_int(n);
                            all_legal_moves.push(legal_move);
                            ()
                        }
                    }
                    all_occupied_bitboard = all_occupied_bitboard >> 1;
                }
                all_legal_moves
            }
        }
    }

    fn unmake_move(&self) -> GameState {
        let maybe_last_move = self.move_history.last();
        match maybe_last_move {
            None => self.clone(),
            Some(move_) => {
                let move_bitboard = move_.to_bitboard();
                let new_turn = match self.turn {
                    Turn::XTurn => Turn::OTurn,
                    Turn::OTurn => Turn::XTurn,
                };
                let new_x_bitboard = match self.turn {
                    Turn::OTurn => self.x_bitboard ^ move_bitboard,
                    Turn::XTurn => self.x_bitboard,
                };
                let new_o_bitboard = match self.turn {
                    Turn::XTurn => self.o_bitboard ^ move_bitboard,
                    Turn::OTurn => self.o_bitboard,
                };
                let new_game_status = GameStatus::InProgress;
                let mut new_move_history = self.move_history.clone();
                new_move_history.pop();

                GameState {
                    x_bitboard: new_x_bitboard,
                    o_bitboard: new_o_bitboard,
                    game_status: new_game_status,
                    move_history: new_move_history,
                    turn: new_turn,
                }
            }
        }
    }

    fn make_move(&self, move_: Move) -> GameState {
        let all_legal_moves = self.all_legal_moves();
        let is_legal_move = all_legal_moves.contains(&move_);
        if !is_legal_move {
            panic!("Illegal move");
        }
        let move_bitboard = move_.to_bitboard();
        let new_x_bitboard = match self.turn {
            Turn::XTurn => self.x_bitboard | move_bitboard,
            Turn::OTurn => self.x_bitboard,
        };
        let new_o_bitboard = match self.turn {
            Turn::XTurn => self.o_bitboard,
            Turn::OTurn => self.o_bitboard | move_bitboard,
        };
        let new_game_status = match self.turn {
            Turn::XTurn => {
                let x_wins = detect_win_condition(new_x_bitboard);
                if x_wins {
                    GameStatus::XWins
                } else {
                    if detect_full_board_condition(new_x_bitboard | new_o_bitboard) {
                        GameStatus::Draw
                    } else {
                        GameStatus::InProgress
                    }
                }
            }
            Turn::OTurn => {
                let o_wins = detect_win_condition(new_o_bitboard);
                if o_wins {
                    GameStatus::OWins
                } else {
                    if detect_full_board_condition(new_x_bitboard | new_o_bitboard) {
                        GameStatus::Draw
                    } else {
                        GameStatus::InProgress
                    }
                }
            }
        };

        let new_turn = match self.turn {
            Turn::XTurn => Turn::OTurn,
            Turn::OTurn => Turn::XTurn,
        };

        let mut new_move_history = self.move_history.clone();
        new_move_history.push(move_);

        GameState {
            x_bitboard: new_x_bitboard,
            o_bitboard: new_o_bitboard,
            game_status: new_game_status,
            move_history: new_move_history,
            turn: new_turn,
        }
    }
}

fn detect_full_board_condition(bitboard: Bitboard) -> bool {
    match bitboard {
        0b111111111 => true,
        _ => false,
    }
}

const WIN_CONDITION_1: Bitboard = 0b111000000;
const WIN_CONDITION_2: Bitboard = 0b000111000;
const WIN_CONDITION_3: Bitboard = 0b000000111;
const WIN_CONDITION_4: Bitboard = 0b100100100;
const WIN_CONDITION_5: Bitboard = 0b010010010;
const WIN_CONDITION_6: Bitboard = 0b001001001;
const WIN_CONDITION_7: Bitboard = 0b100010001;
const WIN_CONDITION_8: Bitboard = 0b001010100;

fn detect_win_condition(bitboard: Bitboard) -> bool {
    let test_1 = match bitboard & WIN_CONDITION_1 {
        WIN_CONDITION_1 => true,
        _ => false,
    };
    let test_2 = match bitboard & WIN_CONDITION_2 {
        WIN_CONDITION_2 => true,
        _ => false,
    };
    let test_3 = match bitboard & WIN_CONDITION_3 {
        WIN_CONDITION_3 => true,
        _ => false,
    };
    let test_4 = match bitboard & WIN_CONDITION_4 {
        WIN_CONDITION_4 => true,
        _ => false,
    };
    let test_5 = match bitboard & WIN_CONDITION_5 {
        WIN_CONDITION_5 => true,
        _ => false,
    };
    let test_6 = match bitboard & WIN_CONDITION_6 {
        WIN_CONDITION_6 => true,
        _ => false,
    };
    let test_7 = match bitboard & WIN_CONDITION_7 {
        WIN_CONDITION_7 => true,
        _ => false,
    };
    let test_8 = match bitboard & WIN_CONDITION_8 {
        WIN_CONDITION_8 => true,
        _ => false,
    };

    test_1 || test_2 || test_3 || test_4 || test_5 || test_6 || test_7 || test_8
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Move {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Move {
    fn from_int(int: u8) -> Move {
        match int {
            1 => Move::One,
            2 => Move::Two,
            3 => Move::Three,
            4 => Move::Four,
            5 => Move::Five,
            6 => Move::Six,
            7 => Move::Seven,
            8 => Move::Eight,
            9 => Move::Nine,
            _ => panic!("shouldnt be possible"),
        }
    }

    fn to_bitboard(&self) -> Bitboard {
        match self {
            Move::One => 0b100000000,
            Move::Two => 0b010000000,
            Move::Three => 0b001000000,
            Move::Four => 0b000100000,
            Move::Five => 0b000010000,
            Move::Six => 0b000001000,
            Move::Seven => 0b000000100,
            Move::Eight => 0b000000010,
            Move::Nine => 0b000000001,
        }
    }

    fn from_bitboard(bitboard: Bitboard) -> Move {
        match bitboard {
            0b100000000 => Move::One,
            0b010000000 => Move::Two,
            0b001000000 => Move::Three,
            0b000100000 => Move::Four,
            0b000010000 => Move::Five,
            0b000001000 => Move::Six,
            0b000000100 => Move::Seven,
            0b000000010 => Move::Eight,
            0b000000001 => Move::Nine,
            _ => panic!("Invalid move"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Turn {
    XTurn,
    OTurn,
}

type Bitboard = u32;

enum Player {
    X,
    O,
}

#[derive(PartialEq, Debug, Clone)]
enum GameStatus {
    XWins,
    OWins,
    Draw,
    InProgress,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_legal_moves_test_1() {
        let game_state = GameState {
            x_bitboard: 0b010011000,
            o_bitboard: 0b001100010,
            game_status: GameStatus::InProgress,
            move_history: Vec::new(),
            turn: Turn::XTurn,
        };

        let result = game_state.all_legal_moves();
        let expected_result = vec![Move::Nine, Move::Seven, Move::One];
        assert_eq!(result, expected_result);
        // assert_eq!(result, 4);
    }

    #[test]
    fn all_legal_moves_initial_position_test() {
        let game_state = GameState {
            x_bitboard: 0b000000000,
            o_bitboard: 0b000000000,
            game_status: GameStatus::InProgress,
            move_history: Vec::new(),
            turn: Turn::XTurn,
        };

        let result = game_state.all_legal_moves();
        let expected_result = vec![
            Move::Nine,
            Move::Eight,
            Move::Seven,
            Move::Six,
            Move::Five,
            Move::Four,
            Move::Three,
            Move::Two,
            Move::One,
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn all_legal_moves_draw_status_test() {
        let game_state = GameState {
            x_bitboard: 0b010011000,
            o_bitboard: 0b001100010,
            game_status: GameStatus::Draw,
            move_history: Vec::new(),
            turn: Turn::XTurn,
        };

        let result = game_state.all_legal_moves();
        let expected_result = Vec::new();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn make_move_initial_position() {
        let game_state = GameState {
            x_bitboard: 0b000000000,
            o_bitboard: 0b000000000,
            game_status: GameStatus::InProgress,
            move_history: Vec::new(),
            turn: Turn::XTurn,
        };

        let move_ = Move::One;
        let result = game_state.make_move(move_);
        let expected_result = GameState {
            x_bitboard: 0b100000000,
            o_bitboard: 0b000000000,
            game_status: GameStatus::InProgress,
            move_history: vec![Move::One],
            turn: Turn::OTurn,
        };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn make_move_that_wins_game() {
        let game_state = GameState {
            x_bitboard: 0b110000000,
            o_bitboard: 0b000000110,
            game_status: GameStatus::InProgress,
            move_history: Vec::new(),
            turn: Turn::XTurn,
        };

        let move_ = Move::Three;
        let result = game_state.make_move(move_);
        let expected_result = GameState {
            x_bitboard: 0b111000000,
            o_bitboard: 0b000000110,
            game_status: GameStatus::XWins,
            move_history: vec![Move::Three],
            turn: Turn::OTurn,
        };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn make_move_that_fills_board_without_winner() {
        let game_state = GameState {
            x_bitboard: 0b101010010,
            o_bitboard: 0b010100101,
            game_status: GameStatus::InProgress,
            move_history: Vec::new(),
            turn: Turn::XTurn,
        };

        let move_ = Move::Six;
        let result = game_state.make_move(move_);
        let expected_result = GameState {
            x_bitboard: 0b101011010,
            o_bitboard: 0b010100101,
            game_status: GameStatus::Draw,
            move_history: vec![Move::Six],
            turn: Turn::OTurn,
        };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn unmake_move_initial_position() {
        let game_state = GameState {
            x_bitboard: 0b000000000,
            o_bitboard: 0b000000000,
            game_status: GameStatus::InProgress,
            move_history: Vec::new(),
            turn: Turn::XTurn,
        };

        let result = game_state.unmake_move();
        let expected_result = game_state.clone();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn unmake_move_after_x_first_move() {
        let game_state = GameState {
            x_bitboard: 0b100000000,
            o_bitboard: 0b000000000,
            game_status: GameStatus::InProgress,
            move_history: vec![Move::One],
            turn: Turn::OTurn,
        };

        let result = game_state.unmake_move();
        let expected_result = GameState {
            x_bitboard: 0b000000000,
            o_bitboard: 0b000000000,
            game_status: GameStatus::InProgress,
            move_history: Vec::new(),
            turn: Turn::XTurn,
        };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn unmake_move_after_x_wins() {
        let game_state = GameState {
            x_bitboard: 0b111000000,
            o_bitboard: 0b000110000,
            game_status: GameStatus::XWins,
            move_history: vec![Move::One, Move::Four, Move::Two, Move::Five, Move::Three],
            turn: Turn::OTurn,
        };

        let result = game_state.unmake_move();
        let expected_result = GameState {
            x_bitboard: 0b110000000,
            o_bitboard: 0b000110000,
            game_status: GameStatus::InProgress,
            move_history: vec![Move::One, Move::Four, Move::Two, Move::Five],
            turn: Turn::XTurn,
        };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn unmake_o_move_in_progress() {
        let game_state = GameState {
            x_bitboard: 0b110000000,
            o_bitboard: 0b000110000,
            game_status: GameStatus::InProgress,
            move_history: vec![Move::One, Move::Four, Move::Two, Move::Five],
            turn: Turn::XTurn,
        };

        let result = game_state.unmake_move();
        let expected_result = GameState {
            x_bitboard: 0b110000000,
            o_bitboard: 0b000100000,
            game_status: GameStatus::InProgress,
            move_history: vec![Move::One, Move::Four, Move::Two],
            turn: Turn::OTurn,
        };
        assert_eq!(result, expected_result);
    }
}
