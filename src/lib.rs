mod pieces;
pub mod terminal;
pub mod board;
use board::ActionB;
use crate::terminal::Print;


pub struct Player{
    color : String,
    name :  String
}
impl Player{
    pub fn new(name:String,color:String)->Self{
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
    log:Vec<ActionB<'a>>
}
impl <'a>Game<'a>{
    pub fn new(player_one:&'a Player,player_two:&'a Player)->Self{
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

        let mut action = self.board.process_command(from,to);
        action.apply(&mut self.board);
        self.log.push(action);
    }
    pub fn get_log(&self)->&Vec<ActionB<'a>>{
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

        assert_eq!(log[0].to_string(),ActionB::Invalid("No piece to take".to_string()).to_string());
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
    assert_eq!(log[0].to_string(),ActionB::Move{from:(0,1),to:(0,2)}.to_string());
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
    assert_eq!(log[0].to_string(),ActionB::Invalid("No piece to take".to_string()).to_string());
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
    assert_eq!(log[0].to_string(),ActionB::Move{from:(0,6),to:(0,5)}.to_string());
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

    assert_eq!(log[0].to_string(),ActionB::Move{from:(0,1),to:(0,2)}.to_string());
    assert_eq!(log[1].to_string(),ActionB::Move{from:(0,2),to:(0,3)}.to_string());
    assert_eq!(log[2].to_string(),ActionB::Move{from:(0,6),to:(0,5)}.to_string());
    assert_eq!(log[3].to_string(),ActionB::Move{from:(0,5),to:(0,4)}.to_string());
    assert_eq!(log[4].to_string(),ActionB::Invalid("Piece in the way".to_string()).to_string());
    assert_eq!(log[4].to_string(),ActionB::Invalid("Piece in the way".to_string()).to_string());
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

    assert_eq!(log[0].to_string(),ActionB::Move{from:(0,1),to:(0,2)}.to_string());
    assert_eq!(log[1].to_string(),ActionB::Invalid("Can not take your own piece".to_string()).to_string());
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

    assert_eq!(log[0].to_string(),ActionB::Move{from:(0,1),to:(0,3)}.to_string());
    assert_eq!(log[1].to_string(),ActionB::Invalid("".to_string()).to_string());
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

    assert_eq!(log[0].to_string(),ActionB::Move{from:(0,1),to:(0,2)}.to_string());
    assert_eq!(log[1].to_string(),ActionB::Invalid("".to_string()).to_string());
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

    assert_eq!(log[0].to_string(),ActionB::Invalid("Something is in the way".to_string()).to_string());
    assert_eq!(log[1].to_string(),ActionB::Invalid("Can not take your own piece".to_string()).to_string());
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

    assert_eq!(log[2].to_string(),ActionB::Move{from:(0,0),to:(0,3)}.to_string());
    assert_eq!(log[3].to_string(),ActionB::Move{from:(0,3),to:(0,0)}.to_string());
    assert_eq!(log[4].to_string(),ActionB::Move{from:(0,0),to:(0,3)}.to_string());
    assert_eq!(log[5].to_string(),ActionB::Move{from:(0,3),to:(7,3)}.to_string());
    assert_eq!(log[6].to_string(),ActionB::Capture{from:(7,3),to:(7,6),temp:None}.to_string());
    assert_eq!(log[7].to_string(),ActionB::Invalid("Something is in the way".to_string()).to_string());
    assert_eq!(log[8].to_string(),ActionB::Move{from:(7,6),to:(7,3)}.to_string());
    assert_eq!(log[9].to_string(),ActionB::Move{from:(7,3),to:(0,3)}.to_string());
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

    assert_eq!(log[0].to_string(),ActionB::Invalid("Can not take your own piece".to_string()).to_string());
    assert_eq!(log[1].to_string(),ActionB::Move{from:(1,0),to:(2,2)}.to_string());
    assert_eq!(log[2].to_string(),ActionB::Invalid("".to_string()).to_string());
    assert_eq!(log[3].to_string(),ActionB::Move{from:(2,2),to:(1,0)}.to_string());
    assert_eq!(log[4].to_string(),ActionB::Move{from:(1,0),to:(0,2)}.to_string());
    assert_eq!(log[5].to_string(),ActionB::Move{from:(0,2),to:(1,4)}.to_string());
    assert_eq!(log[6].to_string(),ActionB::Capture{from:(1,4),to:(2,6),temp:None}.to_string());
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

    assert_eq!(log[0].to_string(),ActionB::Invalid("Something is in the way".to_string()).to_string());
    assert_eq!(log[1].to_string(),ActionB::Move{from:(3,1),to:(3,2)}.to_string());
    assert_eq!(log[2].to_string(),ActionB::Move{from:(2,0),to:(5,3)}.to_string());
    assert_eq!(log[3].to_string(),ActionB::Invalid("Something is in the way".to_string()).to_string());

    assert_eq!(log[4].to_string(),ActionB::Capture{from:(5,3),to:(2,6),temp:None}.to_string());
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

    assert_eq!(log[0].to_string(),ActionB::Move{from:(4,1),to:(4,2)}.to_string());
    assert_eq!(log[1].to_string(),ActionB::Move{from:(3,0),to:(4,1)}.to_string());
    assert_eq!(log[2].to_string(),ActionB::Move{from:(4,1),to:(3,2)}.to_string());
    assert_eq!(log[3].to_string(),ActionB::Move{from:(3,2),to:(3,3)}.to_string());
    assert_eq!(log[4].to_string(),ActionB::Invalid("".to_string()).to_string());
    assert_eq!(log[5].to_string(),ActionB::Invalid("".to_string()).to_string());
    assert_eq!(log[6].to_string(),ActionB::Invalid("".to_string()).to_string());
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

    assert_eq!(log[0].to_string(),ActionB::Move{from:(5,1),to:(5,2)}.to_string());
    assert_eq!(log[1].to_string(),ActionB::Move{from:(4,0),to:(5,1)}.to_string());
    assert_eq!(log[2].to_string(),ActionB::Move{from:(5,1),to:(4,2)}.to_string());
    assert_eq!(log[3].to_string(),ActionB::Move{from:(4,2),to:(4,3)}.to_string());
    assert_eq!(log[4].to_string(),ActionB::Move{from:(4,3),to:(4,5)}.to_string());
    assert_eq!(log[5].to_string(),ActionB::Move{from:(4,5),to:(2,5)}.to_string());
    assert_eq!(log[6].to_string(),ActionB::Move{from:(2,5),to:(0,3)}.to_string());
}
