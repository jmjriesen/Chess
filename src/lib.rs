mod pices;
pub mod terminal;
pub struct Player{
    color : String,
    name :  String
}
impl Player{
    pub fn new(name:String,color:String)->Player{
        Player{
            name:name,
            color:color,
        }
    }
}

pub struct Board<'a>{
    grid :Vec<Vec<Option<Box<dyn pices::Pice+'a>>>>,
    player_one:&'a Player,
    player_two:&'a Player,
}

impl <'b> Board <'b>{
    pub fn new(player_one:&'b Player,player_two:&'b Player)->Board<'b>{

        let mut t = Board{
            grid: Vec::new(),
                player_one:player_one,
            player_two:player_two,
        };

        for x in 0..8{
            t.grid.push(Vec::new());
            for _ in 0..8{
                t.grid[x].push(None);
            }
        }
        for x in 0..8{
            t.grid[1][x] = Some(Box::new(pices::Pawn::new(player_one)));
            t.grid[6][x] = Some(Box::new(pices::Pawn::new(player_two)));
        }

        t.grid[0][0] = Some(Box::new(pices::Rook::new(player_one)));
        t.grid[7][0] = Some(Box::new(pices::Rook::new(player_two)));
        t.grid[0][7] = Some(Box::new(pices::Rook::new(player_one)));
        t.grid[7][7] = Some(Box::new(pices::Rook::new(player_two)));

        t.grid[0][1] = Some(Box::new(pices::Knight::new(player_one)));
        t.grid[7][1] = Some(Box::new(pices::Knight::new(player_two)));
        t.grid[0][6] = Some(Box::new(pices::Knight::new(player_one)));
        t.grid[7][6] = Some(Box::new(pices::Knight::new(player_two)));


        t.grid[0][2] = Some(Box::new(pices::Bishops::new(player_one)));
        t.grid[7][2] = Some(Box::new(pices::Bishops::new(player_two)));
        t.grid[0][5] = Some(Box::new(pices::Bishops::new(player_one)));
        t.grid[7][5] = Some(Box::new(pices::Bishops::new(player_two)));

        t.grid[0][3] = Some(Box::new(pices::Queen::new(player_one)));
        t.grid[7][3] = Some(Box::new(pices::Queen::new(player_two)));
        t.grid[0][4] = Some(Box::new(pices::King::new(player_one)));
        t.grid[7][4] = Some(Box::new(pices::King::new(player_two)));

        t
    }

}

