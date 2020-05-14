use crate::pieces;
use crate::pieces::Pice;
use crate::Player;
pub struct Board<'a>{
      grid :Vec<Vec<Option<Box<dyn pieces::Pice+'a>>>>,
}

impl <'b> Board <'b>{
    pub fn new(player_one:&'b Player,player_two:&'b Player)->Board<'b>{

        let mut t = Board{
            grid: Vec::new(),
        };
        // set up the 2d vector.
        for x in 0..8{
            t.grid.push(Vec::new());
            for _ in 0..8{
                t.grid[x].push(None);
            }
        }
        //placing the pawns
        for (player,row) in [(player_one,1),(player_two,6)].iter(){
            let direction = if *row==1 {1}else{-1};
            for x in 0..8{
                t.grid[*row][x] = Some(Box::new(pieces::Pawn::new(player,direction)));
            }
        }
        //placing symmetric Pieces.
        for (player,row) in [(player_one,0),(player_two,7)].iter(){

            t.grid[*row][0] = Some(Box::new(pieces::Rook::new(player)));
            t.grid[*row][7] = Some(Box::new(pieces::Rook::new(player)));

            t.grid[*row][1] = Some(Box::new(pieces::Knight::new(player)));
            t.grid[*row][6] = Some(Box::new(pieces::Knight::new(player)));

            t.grid[*row][2] = Some(Box::new(pieces::Bishops::new(player)));
            t.grid[*row][5] = Some(Box::new(pieces::Bishops::new(player)));
        }
        //placing asymmetric Pieces.
        t.grid[0][3] = Some(Box::new(pieces::King::new(player_one)));
        t.grid[7][3] = Some(Box::new(pieces::King::new(player_two)));
        t.grid[0][4] = Some(Box::new(pieces::Queen::new(player_one)));
        t.grid[7][4] = Some(Box::new(pieces::Queen::new(player_two)));

        t
    }
    pub fn process_command(& mut self,from:(usize,usize),to:(usize,usize))->Box<dyn Action>{
        match self.get(from){
            None => Box::new(Invalid::new("No piece to move")),
            Some(piece)=> {
                piece.process_command(from,to,&self)
            }
        }
    }
    pub fn get(&self,(x,y):(usize,usize))->&Option<Box<dyn pieces::Pice+'b>>{
        &self.grid[y][x]
    }
    //TODO consider getting rid of this.
    pub fn get_mut(&mut self,(x,y):(usize,usize))->&mut Option<Box<dyn pieces::Pice+'b>>{
        &mut self.grid[y][x]
    }
    pub fn foreach(&self,f:fn(&Option<Box<dyn pieces::Pice+'b>>,usize,usize),g:fn(usize)){
        let mut y = 0;
            for row in &self.grid {
                let mut x =  0;
                for entry in row{
                    f(entry,x,y);
                    x+=1;
                }
                g(y);
                y+=1;
            }
    }
}
//I am transitioning to a command pattern here these will be actions that can modify the board.
pub trait Action{
    fn apply(&self, board:&mut Board);
    fn to_string(&self)->String;
}

pub struct Move{
    from : (usize,usize),
    to : (usize,usize),
}

impl Move{
    pub fn new(from : (usize,usize),to : (usize,usize))->Move{
        Move {from:from,to:to}
    }
}
impl Action for Move{
    fn apply(&self, board:&mut Board){
        let (x_from,y_from) = self.from;
        let (x_to,y_to) = self.to;
        let mut temp = None;
        std::mem::swap(&mut board.grid[y_from][x_from], &mut temp);
        std::mem::swap(&mut board.grid[y_to][x_to], &mut temp);
        match board.get_mut(self.to){
            None =>{panic!(); }, //This should never happen as we just moved this piece.
            Some(x)=> x.make_move(),
        }
    }
    fn to_string(&self)->String{
        format!("Move {:?},{:?}",self.from,self.to).to_string()
    }
}
pub struct Captrue{
    movement : Move,
    captrued : dyn pieces::Pice,
}
impl Action for Captrue{
    fn apply(&self, _board:&mut Board){//TODO
    }
    fn to_string(&self)->String{
        self.movement.to_string() + "Captrue"
    }
}

pub struct Invalid {
    message: &'static str,
}
// TODO resolve the two news
impl Invalid{
    pub fn new(message:&'static str)->Invalid{
        Invalid{message : message}
    }
}
impl Action for Invalid{
    fn apply(&self, _board:&mut Board){//TODO
    }
    fn to_string(&self)->String{
        format!("Invalid {}", self.message).to_string()
    }
}
