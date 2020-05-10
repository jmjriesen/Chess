use crate::pieces;
use crate::Player;
pub struct Board<'a>{
    pub grid :Vec<Vec<Option<Box<dyn pieces::Pice+'a>>>>,
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
        for x in 0..8{
            for (player,row) in [(player_one,1),(player_two,6)].iter(){
                t.grid[*row][x] = Some(Box::new(pieces::Pawn::new(player)));
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
        t.grid[0][3] = Some(Box::new(pieces::Queen::new(player_one)));
        t.grid[7][3] = Some(Box::new(pieces::Queen::new(player_two)));
        t.grid[0][4] = Some(Box::new(pieces::King::new(player_one)));
        t.grid[7][4] = Some(Box::new(pieces::King::new(player_two)));

        t
    }
    pub fn action(& mut self,(x_from,y_from):(usize,usize),(x_to,y_to):(usize,usize)){
        let mut temp = None;
        std::mem::swap(&mut self.grid[x_from][y_from], &mut temp);
        std::mem::swap(&mut self.grid[x_to][y_to], &mut temp);
    }
}
