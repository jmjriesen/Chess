use colored::*;
use crate::pices;
use crate::Board;

pub trait Print{
    fn to_string(&self)->colored::ColoredString;
}

impl <'a>Print for pices::Pawn<'a>{
    fn to_string(&self)->colored::ColoredString{
        "P".to_string().color(self.owner.color.clone())
    }
}
impl <'a>Print for pices::Knight<'a>{
    fn to_string(&self)->colored::ColoredString{
        "R".to_string().color(self.owner.color.clone())
    }
}
impl <'a>Print for pices::Bishops<'a>{
    fn to_string(&self)->colored::ColoredString{
        "N".to_string().color(self.owner.color.clone())
    }
}
impl <'a>Print for pices::Rook<'a>{
    fn to_string(&self)->colored::ColoredString{
        "B".to_string().color(self.owner.color.clone())
    }
}
impl <'a>Print for pices::Queen<'a>{
    fn to_string(&self)->colored::ColoredString{
        "K".to_string().color(self.owner.color.clone())
    }
}
impl <'a>Print for pices::King<'a>{
    fn to_string(&self)->colored::ColoredString{
        "Q".to_string().color(self.owner.color.clone())
    }
}
impl <'a>Print for Board<'a>{
    fn to_string(&self)->colored::ColoredString{
        println!("{} vs {}",self.player_one.name,self.player_two.name);
        let mut tile_paridy = true;
        for row in &self.grid {
            for entry in row{

                let symbol = match entry {
                    None => " ".color("white"),
                    Some(e)=> e.to_string().bold(),
                };
                let background = if tile_paridy {"white"} else {"Black"};

                print!(" {}",symbol.on_color(background));

                tile_paridy = !tile_paridy;
            }
            tile_paridy = !tile_paridy;
            println!("");
        }
        "temp".color("white")
    }
}

#[test]
fn pawn_to_string_test() {
    use crate::Player;
    let jacob = Player::new("Jacob".to_string(),"red".to_string());
    let p = pices::Pawn::new(&jacob);
    assert_eq!(p.to_string(), "P".red());
}
 
