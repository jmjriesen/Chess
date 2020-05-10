use colored::*;
use crate::Player;

pub trait Pice: std::clone::Clone{
    fn to_string(&self)->colored::ColoredString;
}
#[derive(Clone, Copy)]
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
    pub fn to_string(&self)->colored::ColoredString{
        self.symbol.to_string().color(self.owner.color.clone())
    }
}
impl <'a>Pice for Pawn<'a>{
    fn to_string(&self)->colored::ColoredString{
        self.symbol.to_string().color(self.owner.color.clone())
    }
}

