use chess::Player;
use chess::Game;

fn main() {
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    game.action((1,0),(2,0));
    game.display();
}

