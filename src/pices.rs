use colored::*;
use crate::Player;

pub trait Pice{
    fn to_string(&self)->colored::ColoredString;
}
pub struct Pawn<'a>{
    owner : &'a Player,
    symbol : char,
}
impl <'a> Pawn<'a>{
    pub fn new(owner:&Player)->Pawn{
        Pawn{
            owner : owner,
            symbol :'P',
        }
    }
}
impl <'a>Pice for Pawn<'a>{
    fn to_string(&self)->colored::ColoredString{
        self.symbol.to_string().color(self.owner.color.clone())
    }
}

