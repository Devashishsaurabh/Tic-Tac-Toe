const SHOULD_PRINT: bool = false;
const MAX_VALUE: i8 = 100; /* max value possible */


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
        MinMax {
            board: board,
            player: player,
            opponent:  if player == 'X' {'O'} else {'X'},  
        }  
    }

    // the main function that will be called to calulate the best postion
    fn find_best_move(&mut self) -> i8 {
        let mut best_value =  -1-MAX_VALUE;
        let mut best_move =  -1;
        for pos in 0..9 {
            if self.board[pos] == ' ' { 
                self.board[pos] = self.player;
                let predicted_value = self.minmax(0, Player::Minimizer);
                println!("_tree calulcation ended: {}", predicted_value);  
                self.display_board();
                self.board[pos] = ' ';
                if predicted_value > best_value {
                    best_value = predicted_value;
                    best_move = pos as i8;
                    if predicted_value == MAX_VALUE {
                        return best_move;
                    }
                }
            }
        }
        return best_move;
    }

    // function used to evalulate the given position
    fn minmax(&mut self, depth: i8, is_maximizer: Player) -> i8{
        if  !self.is_moves_left() {
            return 0;            
        }
        match self.is_won() {
            'X' => return MAX_VALUE - depth,
            'O' => return depth - MAX_VALUE, 
            ' ' => match is_maximizer {
                Player::Maximizer => return * &self.maximize(depth+1),
                Player::Minimizer => return * &self.minimize(depth+1),
            },   
            _ => panic!("invalid charector")  
        }
    }
}

impl MinMax {
    fn is_moves_left(&self) -> bool {
        self.board.contains(&' ')
    }

    fn display_board(&self) {
        if SHOULD_PRINT {
            println!( "
            {}|{}|{}
            {}|{}|{}
            {}|{}|{}",
            self.board[0], self.board[1], self.board[2],
            self.board[3], self.board[4], self.board[5],
            self.board[6], self.board[7], self.board[8] );
        }
    }

    fn maximize(&mut self, depth: i8) -> i8 {
        let mut best_value = -MAX_VALUE;
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
        let mut best_value = MAX_VALUE;
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

    fn is_won(&self) -> char {
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
            if board[a] == board[b] && board[b] == board[c] && board[a] != ' '{
                return board[a]
            }
        }
        return ' '; 
    }
}




  //////////////////////////
 //  TESTING THE CODE    //
//////////////////////////

#[test]
fn test_moves_left() {
    let board = vec![
        'X', 'O', 'X',
        ' ', 'O', ' ',
        ' ', ' ', ' ',
    ];
    let player = 'X';
    assert_eq!(MinMax::new(board, player).is_moves_left(), true); 
}

#[test]
fn test_moves_not_left() {
    let board = vec![
        'X', 'O', 'X',
        'X', 'O', 'X',
        'O', 'X', 'O',
    ];
    let player = 'X';
    assert_eq!(MinMax::new(board, player).is_moves_left(), false); 
}

#[test]
fn test_o_won() {
    let board = vec![
        'X', 'O', 'X',
        ' ', 'O', 'X',
        'O', 'O', ' ',
    ];
    let player = 'X';
    assert_eq!(MinMax::new(board, player).is_won(), 'O'); 
}

#[test]
fn test_x_won() {
    let board = vec![
        'X', 'O', 'X',
        'X', 'O', 'O',
        'X', ' ', ' ',
    ];
    let player = 'X';
    assert_eq!(MinMax::new(board, player).is_won(), 'X'); 
}

#[test]
fn test_not_won() {
    let board = vec![
        'X', 'O', 'X',
        ' ', 'O', 'X',
        'O', ' ', ' ',
    ];
    let player = 'X';
    assert_eq!(MinMax::new(board, player).is_won(), ' '); 
}

#[test]
fn test_ignore_empty() {
    let board = vec![
        ' ', ' ', ' ',
        ' ', ' ', ' ',
        ' ', ' ', ' ',
    ];
    let player = 'X';
    assert_eq!(MinMax::new(board, player).is_won(), ' '); 
}

#[test]
fn test_x_prevent_win() {
    let board = vec![
        'X', 'O', 'X',
        ' ', 'O', ' ',
        ' ', ' ', ' ',
    ];
    let player = 'X';
    assert_eq!(MinMax::new(board, player).find_best_move(), 7); 
}

#[test]
fn test_x_find_win() {
    let board = vec![
        'X', ' ', 'X',
        ' ',  'O', ' ',
        ' ', 'O', ' ',
    ];
    let player = 'X';
    assert_eq!(MinMax::new(board, player).find_best_move(), 1); 
}

#[test]
fn test_o_prevent_win() {
    let board = vec![
        'X', 'O', 'X',
        ' ', 'O', ' ',
        ' ', ' ', ' ',
    ];
    let player = 'O';
    assert_eq!(MinMax::new(board, player).find_best_move(), 7); 
}

#[test]
fn test_o_find_win() {
    let board = vec![
        'O', ' ', 'O',
        ' ', 'X', ' ',
        'X', ' ', ' ',
    ];
    let player = 'O';
    assert_eq!(MinMax::new(board, player).find_best_move(), 1); 
}