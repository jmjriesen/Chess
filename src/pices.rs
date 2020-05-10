use crate::Player;
use crate::terminal;

pub trait Pice: terminal::Print{}

pub struct Pawn<'a>{
 pub   owner : &'a Player,
}
impl <'a> Pawn<'a>{
    pub fn new(owner:&Player)->Pawn{
        Pawn{
            owner : owner,
        }
    }
}
impl <'a>Pice for Pawn<'a>{
}

pub struct Rook<'a>{
    pub     owner : &'a Player,
}
impl <'a> Rook<'a>{
    pub fn new(owner:&Player)->Rook{
        Rook{
            owner : owner,
        }
    }
}
impl <'a>Pice for Rook<'a>{
}

pub struct Knight<'a>{
    pub      owner : &'a Player,
}
impl <'a> Knight<'a>{
    pub fn new(owner:&Player)->Knight{
        Knight{
            owner : owner,
        }
    }
}
impl <'a>Pice for Knight<'a>{
}

pub struct Bishops<'a>{
pub    owner : &'a Player,
}
impl <'a> Bishops<'a>{
    pub fn new(owner:&Player)->Bishops{
        Bishops{
            owner : owner,
        }
    }
}
impl <'a>Pice for Bishops<'a>{
}

pub struct King<'a>{
    pub       owner : &'a Player,
}
impl <'a> King<'a>{
    pub fn new(owner:&Player)->King{
        King{
            owner : owner,
        }
    }
}
impl <'a>Pice for King<'a>{
}

pub struct Queen<'a>{
    pub       owner : &'a Player,
}
impl <'a> Queen<'a>{
    pub fn new(owner:&Player)->Queen{
        Queen{
            owner : owner,
        }
    }
}
impl <'a>Pice for Queen<'a>{
}
