use chess::Player;
use chess::Board;
use chess::terminal::Print;

fn main() {
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let board = Board::new(&jacob,&tony);
    let _ = board.to_string();
}

