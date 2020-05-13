mod pieces;
pub mod terminal;
pub mod board;
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
}
impl <'a>Game<'a>{
    pub fn new(player_one:&'a Player,player_two:&'a Player)->Game<'a>{
        Game{
            board : board::Board::new(player_one,player_two),
            player_one :player_one,
            player_two :player_two,
        }
    }
    pub fn display(&self){
        println!("{} vs {}",self.player_one.name,self.player_two.name);
        let _ = self.board.to_string();
    }
    pub fn action(& mut self,from:(usize,usize),to:(usize,usize))->Result<(),String>{

        let can_move = self.board.can_move(from,to);

        match can_move {
            Err(a) => Err(a),
            Ok(()) =>{
                match self.board.get_mut(from){
                    None => {panic!();}// something must have gone wrong with can_move
                    Some(piece)=> {piece.make_move();}
                }
                self.board.action( from,to);
                Ok(())
            }
        }
    }
}

#[test]
    fn pawn_movement_test1(){
        let jacob = Player::new("Jacob".to_string(),"red".to_string());
        let tony = Player::new("tony".to_string(),"blue".to_string());
        let mut game= Game::new(&jacob,&tony);
        game.display();
        let result =  game.action((0,1),(1,2));
        game.display();
        assert_eq!(result,Err("Ilegle Move".to_string()));
}

#[test]
fn pawn_movement_test2(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    let result =  game.action((0,1),(0,2));
    game.display();
    assert_eq!(result,Ok(()));
}

#[test]
fn pawn_movement_test3(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    let result =  game.action((0,6),(1,5));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));
}

#[test]
fn pawn_movement_test4(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    let result =  game.action((0,6),(0,5));
    game.display();
    assert_eq!(result,Ok(()));
}
#[test]
fn pawn_movement_test5(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    let result =  game.action((0,1),(0,2));
    game.display();
    assert_eq!(result,Ok(()));
    let result =  game.action((0,2),(0,3));
    game.display();
    assert_eq!(result,Ok(()));
    let result =  game.action((0,6),(0,5));
    game.display();
    assert_eq!(result,Ok(()));
    let result =  game.action((0,5),(0,4));
    game.display();
    assert_eq!(result,Ok(()));
    let result =  game.action((0,4),(0,3));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));
    let result =  game.action((0,3),(0,4));
    assert_eq!(result,Err("Ilegle Move".to_string()));
}

#[test]
fn pawn_movement_test6(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    let result =  game.action((0,1),(0,2));
    assert_eq!(result,Ok(()));
    // attempts to take there own pawn.
    let result =  game.action((1,1),(0,2));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));
}

#[test]
fn pawn_movement_test7(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    let result =  game.action((0,1),(0,3));
    assert_eq!(result,Ok(()));
    // attempts to take there own pawn.
    let result =  game.action((0,3),(0,5));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));
}


#[test]
fn pawn_movement_test8(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    let result =  game.action((0,1),(0,2));
    game.display();
    assert_eq!(result,Ok(()));
    // attempts to take there own pawn.
    let result =  game.action((0,2),(0,4));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));
}

#[test]
fn rook_movement_test1(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    //cant move through a pice
    let result =  game.action((0,0),(0,2));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));
    // can't take your own pice.
    let result =  game.action((0,0),(0,1));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));
}


#[test]
fn rook_movement_test2(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();
    let _result =  game.action((0,1),(0,3));
    let _result =  game.action((0,3),(0,4));
    game.display();
    let result =  game.action((0,0),(0,3));
    game.display();
    assert_eq!(result,Ok(()));
    let result =  game.action((0,3),(0,0));
    game.display();
    assert_eq!(result,Ok(()));
    let result =  game.action((0,0),(0,3));
    game.display();
    assert_eq!(result,Ok(()));
    let result =  game.action((0,3),(7,3));
    game.display();
    assert_eq!(result,Ok(()));
    let result =  game.action((7,3),(7,6));
    game.display();
    assert_eq!(result,Ok(()));
    let result =  game.action((7,6),(3,6));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));
    let result =  game.action((7,6),(7,3));
    game.display();
    assert_eq!(result,Ok(()));
    let result =  game.action((7,3),(0,3));
    game.display();
    assert_eq!(result,Ok(()));
}


#[test]
fn knight_movement_test2(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();

    let result =  game.action((1,0),(3,1));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));

    let result =  game.action((1,0),(2,2));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((2,2),(2,1));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));

    let result =  game.action((2,2),(1,0));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((1,0),(0,2));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((0,2),(1,4));
    game.display();
    assert_eq!(result,Ok(()));
    let result =  game.action((1,4),(2,6));
    game.display();
    assert_eq!(result,Ok(()));
}


#[test]
fn bishops_movement_test2(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();

    let result =  game.action((2,0),(4,2));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));

    let _result =  game.action((3,1),(3,2));
    let result =  game.action((2,0),(5,3));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((5,3),(1,7));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));

    let result =  game.action((5,3),(2,6));
    game.display();
    assert_eq!(result,Ok(()));
}

#[test]
fn king_movement_test2(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();

    let result =  game.action((4,1),(4,2));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((3,0),(4,1));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((4,1),(3,2));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((3,2),(3,3));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((3,3),(3,5));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));

    let result =  game.action((3,3),(1,3));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));

    let result =  game.action((3,3),(5,5));
    game.display();
    assert_eq!(result,Err("Ilegle Move".to_string()));
}


#[test]
fn queen_movement_test2(){
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let tony = Player::new("tony".to_string(),"blue".to_string());
    let mut game= Game::new(&jacob,&tony);
    game.display();

    let result =  game.action((5,1),(5,2));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((4,0),(5,1));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((5,1),(4,2));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((4,2),(4,3));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((4,3),(4,5));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((4,5),(2,5));
    game.display();
    assert_eq!(result,Ok(()));

    let result =  game.action((2,5),(0,3));
    game.display();
    assert_eq!(result,Ok(()));
}
