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
    pub fn action(& mut self,(x_from,y_from):(usize,usize),(x_to,y_to):(usize,usize)){
        self.board.action((x_from,y_from),(x_to,y_to));
    }
}
