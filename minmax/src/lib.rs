enum Player {
    Maximizer,
    Minimizer
} 



struct MinMax {
    board: Vec<char>,
    player: char,
    opponent: char,
}

trait Interface {
    fn new(board: Vec<char>, player: char) -> MinMax; 
    fn find_best_move(&mut self) -> i8;
    fn minmax(&mut self, depth: i8, is_maximizer: Player) -> i8;
}

impl Interface for  MinMax {

    fn new(board: Vec<char>, player: char) -> MinMax {
        let opponent = if player == 'X' {'X'} else {'O'};
        MinMax {
            board: board,
            player: player,
            opponent: opponent,  
        }  
    }

    // the main function that will be called to calulate the best postion
    fn find_best_move(&mut self) -> i8 {
        let mut best_value: i8 = -127;
        let mut best_move =  -1;
        for pos in 0..9 {
            if self.board[pos] == ' ' { 
                self.board[pos] = self.player;
                let predicted_value = self.minmax(0, Player::Minimizer);
                self.board[pos] = ' ';
                if predicted_value > best_value {
                    best_value = predicted_value;
                    best_move = pos as i8;
                }
            }
        }
        return best_move;
    }

    // function used to evalulate the given position
    fn minmax(&mut self, depth: i8, is_maximizer: Player) -> i8{
        
        if  !self.is_moves_left() {
            return 0;
        } else if self.is_won() != 0 {
            return self.is_won();
        }  

        match is_maximizer {
            Player::Maximizer => return * &self.maximize(depth+1),
            Player::Minimizer => return * &self.minimize(depth+1),
        };
    }
}

impl MinMax {
    fn is_moves_left(&self) -> bool {
        self.board.contains(&' ')
    }

    fn maximize(&mut self, depth: i8) -> i8 {
        let mut best_value = -127;
        for i in 1..9 {
            if self.board[i] == ' ' {
                self.board[i] = self.player; 
                let predicted_value = self.minmax(depth+1, Player::Minimizer);
                if predicted_value > best_value {
                    best_value = predicted_value;
                } 
                self.board[i] = ' ';
            }
        }
        best_value - depth
    }

    fn minimize(&mut self, depth: i8) -> i8 {
        let mut best_value = 127;
        for i in 1..9 {
            if self.board[i] == ' ' {
                self.board[i] = self.opponent; 
                let predicted_value = self.minmax(depth+1, Player::Maximizer);
                if predicted_value < best_value {
                    best_value = predicted_value;
                } 
                self.board[i] = ' ';
            }
        }
        best_value - depth
    }

    fn is_won(&self) -> i8 {
        let lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        let board = &self.board;
        for [a, b, c] in lines {
            if board[a] == board[b] && board[b] == board[c] {
                    if   board[a] == self.player {
                        return 10;
                    }
                    else if  board[a] == self.opponent { 
                         return -10
                   }
            }
        }
        return 0;
    }
}




#[test]
fn test_prevent_win() {
    let board = vec![
        'X', 'O', 'X',
        ' ',  'O', ' ',
        ' ', ' ', ' ',
    ];
    let player = 'X';
    assert_eq!(MinMax::new(board, player).find_best_move(), 7); 
}

// #[test]
// fn test_find_winning_pos() {
//     let board = vec![
//         'X', ' ', 'X',
//         ' ',  'O', ' ',
//         ' ', 'O', ' ',
//     ];
//     let player = 'O';
//     assert_eq!(MinMax::new(board, player).find_best_move(), 1); 
// }
