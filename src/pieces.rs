use crate::Player;
use crate::board;
use crate::terminal;

pub trait Pice: terminal::Print{
    fn can_move(&self,from:(usize,usize),to:(usize,usize),board:&board::Board)->bool;
    fn owner(&self)->&Player;
    fn make_move(&mut self);
}

pub struct Pawn<'a>{
 pub owner : &'a Player,
 direction : isize,
    moved : bool
}
impl <'a> Pawn<'a>{
    pub fn new(owner:&Player,direction:isize)->Pawn{
        Pawn{
            owner : owner,
            direction:direction,
                moved : false,
        }
    }
}
impl <'a>Pice for Pawn<'a>{
    fn can_move(&self,from:(usize,usize),to:(usize,usize),board:&board::Board)->bool{

        let (x_delta,y_delta) = find_delta(from,to);

        if y_delta == self.direction && x_delta ==0{
            match board.get(to){
                None => true,
                Some(_)=> false,
        }
        }else if y_delta == 2*self.direction && x_delta == 0 && self.moved == false{
            match board.get(to){
                None => true,
                Some(_)=> false,
            }
        }else if y_delta == self.direction && x_delta ==1{
            match board.get(to){
                None => false,
                Some(to_take)=> to_take.owner()!=self.owner(),
            }
        }else if y_delta == self.direction && x_delta ==-1{
            match board.get(to){
                None => false,
                Some(to_take)=> to_take.owner()!=self.owner(),
            }
        }else{
            false
        }
    }
    fn owner(&self)->&Player{
        self.owner
    }
    fn make_move(&mut self){
        self.moved = true;
    }
}

pub struct Rook<'a>{
    pub     owner : &'a Player,
    moved : bool
}
impl <'a> Rook<'a>{
    pub fn new(owner:&Player)->Rook{
        Rook{
            owner : owner,
            moved : false,
        }
    }
}
impl <'a>Pice for Rook<'a>{
    fn can_move(&self,from:(usize,usize),to:(usize,usize),board:&board::Board)->bool{
        let (x_delta,y_delta) = find_delta(from,to);
        if x_delta ==0{
            if y_delta < 0 {
                path_clear(from,(0,-1),y_delta.abs() as usize,board,self.owner())
            }else{
                path_clear(from,(0,1),y_delta.abs() as usize,board,self.owner())
            }
        }else if y_delta ==0{
            if x_delta < 0 {
                path_clear(from,(-1,0),x_delta.abs() as usize,board,self.owner())
            }else{
                path_clear(from,(1,0),x_delta.abs() as usize,board,self.owner())
            }
        }else{
            false
        }
    }
    fn owner(&self)->&Player{
        self.owner
    }
    fn make_move(&mut self){
        self.moved = true
    }
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
    fn can_move(&self,from:(usize,usize),to:(usize,usize),board:&board::Board)->bool{
        let (x_delta,y_delta) = find_delta(from,to);
        if (x_delta.abs() == 2 && y_delta.abs() == 1) ||( x_delta.abs() == 1 && y_delta.abs() == 2){
         path_clear(from,(x_delta,y_delta),1,board,self.owner())
        }else{
            false
        }
    }
    fn owner(&self)->&Player{
        self.owner
    }
    fn make_move(&mut self){}
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
    fn can_move(&self,from:(usize,usize),to:(usize,usize),board:&board::Board)->bool{
        let (x_delta,y_delta) = find_delta(from,to);

        if x_delta == y_delta{
            if x_delta <= 0{
                path_clear(from,(-1,-1),x_delta.abs() as usize,board,self.owner())
            }else{
                path_clear(from,(1,1),x_delta.abs() as usize,board,self.owner())
            }
        }else if x_delta == -y_delta{
            if x_delta <= 0{
                path_clear(from,(-1,1),x_delta.abs() as usize,board,self.owner())
            }else{
                path_clear(from,(1,-1),x_delta.abs() as usize,board,self.owner())
            }
        }else{
            false
        }
    }
    fn owner(&self)->&Player{
        self.owner
    }
    fn make_move(&mut self){}
}

pub struct King<'a>{
    pub       owner : &'a Player,
    moved : bool
}
impl <'a> King<'a>{
    pub fn new(owner:&Player)->King{
        King{
            owner : owner,
            moved : false,
        }
    }
}
impl <'a>Pice for King<'a>{
    fn can_move(&self,from:(usize,usize),to:(usize,usize),board:&board::Board)->bool{
        let (x_delta,y_delta) = find_delta(from,to);
        if (x_delta.abs() == 1 || x_delta.abs() == 0) && (y_delta.abs() == 1 || y_delta.abs() == 0){
            path_clear(from,(x_delta,y_delta),1,board,self.owner())
        }else{
            false
        }
        //TODO Castling
    }
    fn owner(&self)->&Player{
        self.owner
    }
    fn make_move(&mut self){
        self.moved  = true;
    }
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
    fn can_move(&self,from:(usize,usize),to:(usize,usize),board:&board::Board)->bool{
        println!("Much Other");
        let (x_delta,y_delta) = find_delta(from,to);
        if x_delta == y_delta{
            if x_delta <= 0{
                path_clear(from,(-1,-1),x_delta.abs() as usize,board,self.owner())
            }else{
                path_clear(from,(1,1),x_delta.abs() as usize,board,self.owner())
            }
        }else if x_delta == -y_delta{
            if x_delta <= 0{
                path_clear(from,(-1,1),x_delta.abs() as usize,board,self.owner())
            }else{
                path_clear(from,(1,-1),x_delta.abs() as usize,board,self.owner())
            }
        }else if x_delta ==0{
            if y_delta < 0 {
                path_clear(from,(0,-1),y_delta.abs() as usize,board,self.owner())
            }else{
                path_clear(from,(0,1),y_delta.abs() as usize,board,self.owner())
            }
        }else if y_delta ==0{
            if x_delta < 0 {
                path_clear(from,(-1,0),x_delta.abs() as usize,board,self.owner())
            }else{
                path_clear(from,(1,0),x_delta.abs() as usize,board,self.owner())
            }
        }else{
            false
        }
    }
    fn owner(&self)->&Player{
        self.owner
    }
    fn make_move(&mut self){}
}

fn find_delta(from:(usize,usize),to:(usize,usize))->(isize,isize){
    let (x_from,y_from) = from;
    let (x_to,y_to) = to;
    let x_delta = x_to as isize - x_from as isize;
    let y_delta = y_to as isize - y_from as isize;
    (x_delta,y_delta)
}
fn path_clear(start:(usize,usize),derection:(isize,isize),length:usize,board:&board::Board,player:&Player)->bool{
    let mut path_clear = true;
    for i in 1..length{
        match board.get(compute_pose(start,derection,i)){
            None =>{},
            Some(_) => {path_clear = false; break;},
        };
    }
    if path_clear {
        match board.get(compute_pose(start,derection,length)){
            None => true,
            Some(target) => target.owner() != player,
        }
    }else{
        false
    }
}

fn compute_pose(start:(usize,usize),derection:(isize,isize),length:usize)->(usize,usize){
    let (x,y) = start;
    let (x_delta,y_delta) = derection;
    let new_x = x as isize + length as isize *x_delta;
    let new_y = y as isize + length as isize *y_delta;
    (new_x as usize,new_y as usize)
}
