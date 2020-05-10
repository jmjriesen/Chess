use colored::*;
use crate::pieces;
use crate::board::Board;

pub trait Print{
    fn to_string(&self)->colored::ColoredString;
}

impl <'a>Print for pieces::Pawn<'a>{
    fn to_string(&self)->colored::ColoredString{
        "P".to_string().color(self.owner.color.clone())
    }
}
impl <'a>Print for pieces::Knight<'a>{
    fn to_string(&self)->colored::ColoredString{
        "N".to_string().color(self.owner.color.clone())
    }
}
impl <'a>Print for pieces::Bishops<'a>{
    fn to_string(&self)->colored::ColoredString{
        "B".to_string().color(self.owner.color.clone())
    }
}
impl <'a>Print for pieces::Rook<'a>{
    fn to_string(&self)->colored::ColoredString{
        "R".to_string().color(self.owner.color.clone())
    }
}
impl <'a>Print for pieces::Queen<'a>{
    fn to_string(&self)->colored::ColoredString{
        "K".to_string().color(self.owner.color.clone())
    }
}
impl <'a>Print for pieces::King<'a>{
    fn to_string(&self)->colored::ColoredString{
        "Q".to_string().color(self.owner.color.clone())
    }
}
impl <'a>Print for Board<'a>{
    fn to_string(&self)->colored::ColoredString{
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
    let p = pieces::Pawn::new(&jacob);
    assert_eq!(p.to_string(), "P".red());
}
 
