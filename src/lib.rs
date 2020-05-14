mod pieces;
pub mod terminal;
pub mod board;
use board::Action;
use crate::terminal::Print;

pub struct Player{
    color : String,
    name :  String
}
impl Player{
    pub fn new(name:String,color:String)->Player{
        Player{
            name:name,
            color:color,
        }
    }
}
impl std::cmp::PartialEq for Player{
    fn eq(&self, other: &Player) -> bool {
        self.name ==  other.name && self.color == other.color
    }
}

pub struct Game<'a>{
    player_one: &'a Player,
    player_two: &'a Player,
    board:board::Board<'a>,
    log:Vec<Box<dyn Action>>
}
impl <'a>Game<'a>{
    pub fn new(player_one:&'a Player,player_two:&'a Player)->Game<'a>{
        Game{
            board : board::Board::new(player_one,player_two),
            player_one :player_one,
            player_two :player_two,
            log : Vec::new(),
        }
    }

    pub fn display(&self){
        println!("{} vs {}",self.player_one.name,self.player_two.name);
        let _ = self.board.to_string();
    }
    pub fn action(& mut self,from:(usize,usize),to:(usize,usize)){

        let action = self.board.process_command(from,to);
        action.apply(&mut self.board);
        self.log.push(action)
    }
    pub fn get_log(&self)->&Vec<Box<dyn Action>>{
        &self.log
    }
}





#[test]
    fn pawn_movement_test1(){
        let jacob = Player::new("Jacob".to_string(),"red".to_string());
        let tony = Player::new("tony".to_string(),"blue".to_string());
        let mut game= Game::new(&jacob,&tony);
        game.display();
        game.action((0,1),(1,2));
        game.display();
        let log = game.get_log();

        assert_eq!(log[0].to_string(),board::Invalid::new("No piece to take").to_string());
}
#[test]
fn pawn_movement_test2(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    game.action((0,1),(0,2));
    game.display();
    let log = game.get_log();
    assert_eq!(log[0].to_string(),board::Move::new((0,1),(0,2)).to_string());
}
#[test]
fn pawn_movement_test3(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    game.action((0,6),(1,5));
    game.display();
    let log = game.get_log();
    assert_eq!(log[0].to_string(),board::Invalid::new("No piece to take").to_string());
}

#[test]
fn pawn_movement_test4(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    game.action((0,6),(0,5));
    game.display();
    let log = game.get_log();
    assert_eq!(log[0].to_string(),board::Move::new((0,6),(0,5)).to_string());
}
#[test]
fn pawn_movement_test5(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    game.action((0,1),(0,2));
    game.display();
    game.action((0,2),(0,3));
    game.display();
    game.action((0,6),(0,5));
    game.display();
    game.action((0,5),(0,4));
    game.display();
    game.action((0,4),(0,3));
    game.display();
    game.action((0,3),(0,4));

    let log = game.get_log();

    assert_eq!(log[0].to_string(),board::Move::new((0,1),(0,2)).to_string());
    assert_eq!(log[1].to_string(),board::Move::new((0,2),(0,3)).to_string());
    assert_eq!(log[2].to_string(),board::Move::new((0,6),(0,5)).to_string());
    assert_eq!(log[3].to_string(),board::Move::new((0,5),(0,4)).to_string());
    assert_eq!(log[4].to_string(),board::Invalid::new("Piece in the way").to_string());
    assert_eq!(log[4].to_string(),board::Invalid::new("Piece in the way").to_string());
}

#[test]
fn pawn_movement_test6(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    game.action((0,1),(0,2));
    // attempts to take there own pawn.
    game.action((1,1),(0,2));
    game.display();

    let log = game.get_log();

    assert_eq!(log[0].to_string(),board::Move::new((0,1),(0,2)).to_string());
    assert_eq!(log[1].to_string(),board::Invalid::new("Can not take your own piece").to_string());
}

#[test]
fn pawn_movement_test7(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    game.action((0,1),(0,3));
    game.action((0,3),(0,5));
    game.display();

    let log = game.get_log();

    assert_eq!(log[0].to_string(),board::Move::new((0,1),(0,3)).to_string());
    assert_eq!(log[1].to_string(),board::Invalid::new("").to_string());
}


#[test]
fn pawn_movement_test8(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    game.action((0,1),(0,2));
    game.display();
    game.action((0,2),(0,4));
    game.display();

    let log = game.get_log();

    assert_eq!(log[0].to_string(),board::Move::new((0,1),(0,2)).to_string());
    assert_eq!(log[1].to_string(),board::Invalid::new("").to_string());
}

#[test]
fn rook_movement_test1(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    //cant move through a pice
    game.action((0,0),(0,2));
    game.display();
    // can't take your own pice.
    game.action((0,0),(0,1));
    game.display();

    let log = game.get_log();

    assert_eq!(log[0].to_string(),board::Invalid::new("Something is in the way").to_string());
    assert_eq!(log[1].to_string(),board::Invalid::new("Can not take your own piece").to_string());
}


#[test]
fn rook_movement_test2(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    game.action((0,1),(0,3));
    game.action((0,3),(0,4));
    game.display();

    game.action((0,0),(0,3));
    game.display();
    game.action((0,3),(0,0));
    game.display();
    game.action((0,0),(0,3));
    game.display();
    game.action((0,3),(7,3));
    game.display();
    game.action((7,3),(7,6));
    game.display();
    game.action((7,6),(3,6));
    game.display();
    game.action((7,6),(7,3));
    game.display();
    game.action((7,3),(0,3));
    game.display();

    let log = game.get_log();

    assert_eq!(log[2].to_string(),board::Move::new((0,0),(0,3)).to_string());
    assert_eq!(log[3].to_string(),board::Move::new((0,3),(0,0)).to_string());
    assert_eq!(log[4].to_string(),board::Move::new((0,0),(0,3)).to_string());
    assert_eq!(log[5].to_string(),board::Move::new((0,3),(7,3)).to_string());
    assert_eq!(log[6].to_string(),board::Move::new((7,3),(7,6)).to_string());
    assert_eq!(log[7].to_string(),board::Invalid::new("Something is in the way").to_string());
    assert_eq!(log[8].to_string(),board::Move::new((7,6),(7,3)).to_string());
    assert_eq!(log[9].to_string(),board::Move::new((7,3),(0,3)).to_string());
}


#[test]
fn knight_movement_test2(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();

    game.action((1,0),(3,1));
    game.display();

    game.action((1,0),(2,2));
    game.display();

    game.action((2,2),(2,1));
    game.display();

    game.action((2,2),(1,0));
    game.display();

    game.action((1,0),(0,2));
    game.display();

    game.action((0,2),(1,4));
    game.display();
    game.action((1,4),(2,6));
    game.display();

    let log = game.get_log();

    assert_eq!(log[0].to_string(),board::Invalid::new("Can not take your own piece").to_string());
    assert_eq!(log[1].to_string(),board::Move::new((1,0),(2,2)).to_string());
    assert_eq!(log[2].to_string(),board::Invalid::new("").to_string());
    assert_eq!(log[3].to_string(),board::Move::new((2,2),(1,0)).to_string());
    assert_eq!(log[4].to_string(),board::Move::new((1,0),(0,2)).to_string());
    assert_eq!(log[5].to_string(),board::Move::new((0,2),(1,4)).to_string());
    assert_eq!(log[6].to_string(),board::Move::new((1,4),(2,6)).to_string());
}


#[test]
fn bishops_movement_test2(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();

    game.action((2,0),(4,2));
    game.display();

    game.action((3,1),(3,2));
    game.action((2,0),(5,3));
    game.display();

    game.action((5,3),(1,7));
    game.display();

    game.action((5,3),(2,6));
    game.display();

    let log = game.get_log();

    assert_eq!(log[0].to_string(),board::Invalid::new("Something is in the way").to_string());
    assert_eq!(log[1].to_string(),board::Move::new((3,1),(3,2)).to_string());
    assert_eq!(log[2].to_string(),board::Move::new((2,0),(5,3)).to_string());
    assert_eq!(log[3].to_string(),board::Invalid::new("Something is in the way").to_string());

    assert_eq!(log[4].to_string(),board::Move::new((5,3),(2,6)).to_string());
}

#[test]
fn king_movement_test2(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();

    game.action((4,1),(4,2));
    game.display();

    game.action((3,0),(4,1));
    game.display();

    game.action((4,1),(3,2));
    game.display();

    game.action((3,2),(3,3));
    game.display();

    game.action((3,3),(3,5));
    game.display();

    game.action((3,3),(1,3));
    game.display();

    game.action((3,3),(5,5));
    game.display();

    let log = game.get_log();

    assert_eq!(log[0].to_string(),board::Move::new((4,1),(4,2)).to_string());
    assert_eq!(log[1].to_string(),board::Move::new((3,0),(4,1)).to_string());
    assert_eq!(log[2].to_string(),board::Move::new((4,1),(3,2)).to_string());
    assert_eq!(log[3].to_string(),board::Move::new((3,2),(3,3)).to_string());
    assert_eq!(log[4].to_string(),board::Invalid::new("").to_string());
    assert_eq!(log[5].to_string(),board::Invalid::new("").to_string());
    assert_eq!(log[6].to_string(),board::Invalid::new("").to_string());
}


#[test]
fn queen_movement_test2(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();

    game.action((5,1),(5,2));
    game.display();

    game.action((4,0),(5,1));
    game.display();

    game.action((5,1),(4,2));
    game.display();

    game.action((4,2),(4,3));
    game.display();

    game.action((4,3),(4,5));
    game.display();

    game.action((4,5),(2,5));
    game.display();

    game.action((2,5),(0,3));
    game.display();

    let log = game.get_log();

    assert_eq!(log[0].to_string(),board::Move::new((5,1),(5,2)).to_string());
    assert_eq!(log[1].to_string(),board::Move::new((4,0),(5,1)).to_string());
    assert_eq!(log[2].to_string(),board::Move::new((5,1),(4,2)).to_string());
    assert_eq!(log[3].to_string(),board::Move::new((4,2),(4,3)).to_string());
    assert_eq!(log[4].to_string(),board::Move::new((4,3),(4,5)).to_string());
    assert_eq!(log[5].to_string(),board::Move::new((4,5),(2,5)).to_string());
    assert_eq!(log[6].to_string(),board::Move::new((2,5),(0,3)).to_string());
}
