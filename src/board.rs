use crate::pieces;
use crate::Player;

pub type Point = (usize,usize);

pub struct Board<'a>{
      grid :Vec<Vec<Option<Box<dyn pieces::Pice<'a> +'a>>>>,
}

impl <'b> Board <'b>{
    pub fn new(player_one:&'b Player,player_two:&'b Player)->Self{

        let mut t :Board<'b>= Board{
            grid: Vec::new(),
        };
        // set up the 2d vector.
        for x in 0..8{
            t.grid.push(Vec::new());
            for _ in 0..8{
                t.grid[x].push(None);
            }
        }
        for entry in [(player_one,1),(player_two,6)].iter(){
            let (player,row): (&'b Player,usize) = *entry;
            let direction = if row==1 {1}else{-1};
            for x in 0..8{
                t.grid[row][x] = Some(Box::new(pieces::Pawn::new(player,direction)));
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
    pub fn process_command(&mut self,from:(usize,usize),to:(usize,usize))->ActionB<'b>{
        match self.get(from){
            None => ActionB::Invalid("No piece to move".to_string()),
            Some(piece)=> {
                piece.process_command(from,to,&self)
            }
        }
    }
    pub fn get(&self,(x,y):(usize,usize))->&std::option::Option<std::boxed::Box<(dyn pieces::Pice<'b> + 'b)>>{
        &self.grid[y][x]
    }

    pub fn take(&mut self,(x,y):Point)->std::option::Option<std::boxed::Box<(dyn pieces::Pice<'b> + 'b)>>{
        let mut temp = None;
        std::mem::swap(&mut self.grid[y][x], &mut temp);
        temp
    }

    pub fn set(&mut self,(x,y):Point, entry :std::option::Option<std::boxed::Box<(dyn pieces::Pice<'b> + 'b)>>){
        let mut temp = entry;
        std::mem::swap(&mut self.grid[y][x], &mut temp);
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
pub enum ActionB<'b>{
    Move{from:Point,to:Point},
    Capture{from:Point,to:Point,temp:Option<Box<dyn pieces::Pice<'b>+'b>>},
    Invalid(String)
}
impl<'b> ActionB<'b>{
    pub fn to_string(&self)->String{
        let string = match &self{
            ActionB::Move{from,to}=> format!("Move from {:?} to {:?}",from,to),
            ActionB::Capture{from,to,temp:_}=>format!("Capture from {:?} to {:?}",from,to),
            ActionB::Invalid(s)=> format!("Invalid {}",s),
        };
        string.to_string()
    }
    pub fn apply(&mut self,board: &mut Board<'b>){
        let transport = |from:Point, to:Point,board:&mut Board<'b>|{
            let mut piece = board.take(from);
            match &mut piece{
                None => {panic!();},
                Some(data) => {data.make_move();},
            }
            board.set(to, piece);
        };

        match self{
            ActionB::Move{from,to}=>{
                transport(*from,*to,board);
            },
            ActionB::Capture{from,to,temp}=>{
                *temp = board.take(*to);
                transport(*from,*to,board);
            },
            ActionB::Invalid(_)=>{ },
        }
    }
}
