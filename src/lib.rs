use colored::*;
mod pices;
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

pub struct Board<'a>{
    grid :Vec<Vec<Option<pices::Pawn<'a>>>>,
    player_one:&'a Player,
    player_two:&'a Player,
}

impl <'a> Board <'a>{
    pub fn new<'b>(player_one:&'b Player,player_two:&'b Player)->Board<'b>{

        let mut t = Board{
            grid: vec![vec![None; 8]; 8],
                player_one:player_one,
            player_two:player_two,
        };

        for x in 0..8{
            t.grid[1][x] = Some(pices::Pawn::new(player_one));
            t.grid[8-2][x] = Some(pices::Pawn::new(player_two));
        }
        t
    }
    pub fn print(&self){
        println!("{} vs {}",self.player_one.name,self.player_two.name);
        for row in &self.grid {
            for entry in row{
                let string= match entry {
                    None => "X".color("white"),
                    Some(e)=> e.to_string(),
                };
                print!(" {}",string);
            }
            println!("");
        }
    }
}
    #[test]
    fn pawn_to_string_test() {

        let jacob = Player::new("Jacob".to_string(),"red".to_string());
        let p  = pices::Pawn::new(&jacob);
        assert_eq!(p.to_string(), "P".red());
    }
    #[test]
    fn board_printing_test(){
        let jacob = Player::new("Jacob".to_string(),"red".to_string());
        let tony = Player::new("tony".to_string(),"blue".to_string());
        let board = Board::new(&jacob,&tony);
        board.print();
        panic!();
    }

