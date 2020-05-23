use crate::Player;
use crate::board;
use crate::board::ActionB;
use crate::board::Point;
use crate::terminal;


pub trait Pice<'a>: terminal::Print{
    fn process_command(&self,from: Point,to:Point,board:&board::Board<'a>)->ActionB<'a>;
    fn owner(&self)->&Player;
    fn make_move(&mut self);
}

pub struct Pawn<'a>{
    pub owner : &'a Player,
    direction : isize,
    moved : bool
}
impl <'a> Pawn<'a>{
    pub fn new(owner:&'a Player,direction:isize)->Pawn{
        Pawn{
            owner : owner,
            direction:direction,
            moved : false,
        }
    }
}
impl <'a>Pice<'a> for Pawn<'a>{
        fn process_command(&self,from: Point,to:Point,board:&board::Board<'a>)->ActionB<'a>{
        let (x_delta,y_delta) = find_delta(from,to);

        if y_delta == self.direction && x_delta ==0{
            match board.get(to){
                None => ActionB::Move{from :from,to:to},
                Some(_)=> ActionB::Invalid("Piece in the way".to_string()),
            }
        }else if y_delta == 2*self.direction && x_delta == 0 && self.moved == false{
            match board.get(to){
                None => ActionB::Move{from :from,to:to},
                Some(_)=> ActionB::Invalid("Piece in the way".to_string()),
            }
        }else if y_delta == self.direction && (x_delta ==1 || x_delta ==-1){
            match board.get(to){
                None => ActionB::Invalid("No piece to take".to_string()),
                Some(to_take)=> {
                    if to_take.owner()!=self.owner(){
                        ActionB::Capture{from:from,to:to,temp:None}
                    }else {
                        ActionB::Invalid("Can not take your own piece".to_string())
                    }
                },
            }
        }else{
            ActionB::Invalid("".to_string())
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
impl <'a>Pice<'a> for Rook<'a>{
    fn process_command(&self,from:Point,to:Point,board:&board::Board<'a>)->ActionB<'a>{
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
            ActionB::Invalid("".to_string())
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
impl <'a>Pice<'a> for Knight<'a>{
    fn process_command(&self,from: Point,to:Point,board:&board::Board<'a>)->ActionB<'a>{
        let (x_delta,y_delta) = find_delta(from,to);
        if (x_delta.abs() == 2 && y_delta.abs() == 1) ||( x_delta.abs() == 1 && y_delta.abs() == 2){
            path_clear(from,(x_delta,y_delta),1,board,self.owner())
        }else{
            ActionB::Invalid("".to_string())
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
impl <'a>Pice<'a> for Bishops<'a>{
        fn process_command(&self,from: Point,to:Point,board:&board::Board<'a>)->ActionB<'a>{
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
            ActionB::Invalid("".to_string())
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
impl <'a>Pice<'a> for King<'a>{
        fn process_command(&self,from: Point,to:Point,board:&board::Board<'a>)->ActionB<'a>{
        let (x_delta,y_delta) = find_delta(from,to);
        if (x_delta.abs() == 1 || x_delta.abs() == 0) && (y_delta.abs() == 1 || y_delta.abs() == 0){
            path_clear(from,(x_delta,y_delta),1,board,self.owner())
        }else{
                ActionB::Invalid("".to_string())
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
impl <'a>Pice<'a> for Queen<'a>{
        fn process_command(&self,from: Point,to:Point,board:&board::Board<'a>)->ActionB<'a>{
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
            ActionB::Invalid("".to_string())
        }
    }
    fn owner(&self)->&Player{
        self.owner
    }
    fn make_move(&mut self){}
}

fn find_delta(from:Point,to:Point)->(isize,isize){
    let (x_from,y_from) = from;
    let (x_to,y_to) = to;
    let x_delta = x_to as isize - x_from as isize;
    let y_delta = y_to as isize - y_from as isize;
    (x_delta,y_delta)
}
fn path_clear<'a>(start:Point,derection:(isize,isize),length:usize,board:&board::Board<'a>,player:&Player)
                  ->ActionB<'a>{
    let mut path_clear = true;
    for i in 1..length{
        match board.get(compute_pose(start,derection,i)){
            None =>{ },
            Some(_) => {path_clear = false; break;},
        };
    }
    if path_clear {
        let to = compute_pose(start,derection,length);
        match board.get(to){
            None => ActionB::Move{from:start,to:to},
            Some(to_take)=> {
                if to_take.owner()!=player {
                    ActionB::Capture{from:start,to:to,temp:None}
                }else {
                    ActionB::Invalid("Can not take your own piece".to_string())
                }
            },
        }
    }else{
        ActionB::Invalid("Something is in the way".to_string())
    }
}

fn compute_pose(start:Point,derection:(isize,isize),length:usize)->Point{
    let (x,y) = start;
    let (x_delta,y_delta) = derection;
    let new_x = x as isize + length as isize *x_delta;
    let new_y = y as isize + length as isize *y_delta;
    (new_x as usize,new_y as usize)
}
