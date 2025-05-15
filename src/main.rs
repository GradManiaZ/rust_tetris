use rand::prelude::*;
use core::array::from_fn;
use std::char;


enum Configuration{
    RL,
    LL,
    TT,
    RS,
    LS,
    SL,
    SS,
    BB
}

struct Piece{
    p_type    :  Configuration,
    config    :  [[bool;4];4],
    rotation  :  usize
}
// [
//        [false,false,false,false],
//        [false,false,false,false],
//        [false,false,false,false],
//        [false,false,false,false]
//    ]
impl Piece {
    const BB: [[bool;4];4] =[
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false]];



    const LL:[[[bool;4];4];4] = [[
        [false,false,false,false],
        [true ,false,false,false],
        [true ,false,false,false],
        [true ,true ,false,false]
    ],[
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false]
    ],[
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false]
    ],[
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false]
    ]];




    const RL:[[[bool;4];4];4] = [[
        [false,false,false,false],
        [false,true ,false,false],
        [false,true ,false,false],
        [true ,true ,false,false]
    ],[
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false]
    ],[  
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false]
    ],[  
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false]
    ]];




    const TT:[[[bool;4];4];4]= [[
        [false,false,false,false],
        [false,false,false,false],
        [false,true ,false,false],
        [true ,true ,true ,false]
    ],[
        [false,false,false,false],
        [false,true ,false,false],
        [true ,true ,false,false],
        [false,true ,false,false]
    ],[
        [false,false,false,false],
        [false,false,false,false],
        [true ,true ,true ,false],
        [false,true ,false,false]
    ],[
        [false,false,false,false],
        [true ,false,false,false],
        [true ,true ,false,false],
        [true ,false,false,false]
    ]];




    const LS:[[[bool;4];4];2]= [[
        [false,false,false,false],
        [true ,false,false,false],
        [true ,true ,false,false],
        [false,true ,false,false]
    ],[
        [false,false,false,false],
        [false,false,false,false],
        [false,true ,true ,false],
        [true ,true ,false,false]
    ]];




    const RS:[[[bool;4];4];2]= [[
        [false,false,false,false],
        [false,true ,false,false],
        [true ,true ,false,false],
        [true ,false,false,false]
    ],[
        [false,false,false,false],
        [false,false,false,false],
        [true ,true ,false,false],
        [false,true ,true ,false]
    ]];




    const SL:[[[bool;4];4];2] = [[
        [true ,false,false,false],
        [true ,false,false,false],
        [true ,false,false,false],
        [true ,false,false,false]
    ],[
        [false,false,false,false],
        [false,false,false,false],
        [false,false,false,false],
        [true ,true ,true ,true ]
    ]];




    const SS:[[[bool;4];4];1] = [[
        [false,false,false,false],
        [false,false,false,false],
        [true ,true ,false,false],
        [true ,true ,false,false]
    ]];



    fn get_configuration(config:Configuration, rotation:usize) -> [[bool;4];4]
    {
        assert!(rotation < 4, "Function: Get Configuration. Piece Rotation Parsing");
        match config{
            Configuration::RL => {
                Piece::RL[rotation]
            },
            Configuration::LL => {
                Piece::LL[rotation]
            },
            Configuration::TT => {
                Piece::TT[rotation]
            },
            Configuration::RS => {
                match rotation{
                    0|2 => {
                        Piece::RS[0]
                    },
                    1|3 => {
                        Piece::RS[1]
                    }
                    _ => {
                        panic!("Function: Get Configuration. Piece Rotation Parsing [RS]")
                    }
                }
            },
            Configuration::LS => {
                match rotation{
                    0|2 => {
                        Piece::RS[0]
                    },
                    1|3 => {
                        Piece::RS[1]
                    }
                    _ => {
                        panic!("Function: Get Configuration. Piece Rotation Parsing [RS]")
                    }
                }
            },
            Configuration::SL => {
                match rotation{
                    0|2 => {
                        Piece::SL[0]
                    },
                    1|3 => {
                        Piece::SL[1]
                    }
                    _ => {
                        panic!("Function: Get Configuration. Piece Rotation Parsing [SL]")
                    }
                }
            },
            Configuration::SS => {
                match rotation{
                    0|1|2|3 => {
                        Piece::SS[0]
                    }
                    _ => {
                        panic!("Function: Get Configuration. Piece Rotation Parsing [SS]")
                    }
                }
            },
            Configuration::BB => {
                Piece::BB
            }
        }
    }   
}

 
struct GameScreen{
    game_board      :  [[bool; GameScreen::logic_width]; GameScreen::logic_height],
    rendered_board  :  [[char; GameScreen::screen_width];GameScreen::screen_height],
    pieces          :  [Piece; GameScreen::piece_limit] // GameScreen::logic_width * 20 tiles, 4 tiles per block max, 1 redundant
}

impl GameScreen {

    //Screen Size
    const screen_width  :  usize  =  24;
    const screen_height :  usize  =  22;


    //State Machine Size
    const logic_width   :  usize  = (GameScreen::screen_width  -4) /2;
    const logic_height  :  usize  = (GameScreen::screen_height -2);
    const piece_limit   :  usize  = (GameScreen::logic_width * GameScreen::logic_height) /4 +1; 
    

    //Rendering configuration
    const B0L: char = '<'; // GameScreen::B0L
    const B1M: char = '|'; // GameScreen::B1M
    const B3R: char = '>'; // GameScreen::B3R
 
    const BC0: char = '.'; // GameScreen::BC0
    const BC1: char = ' '; // GameScreen::BC1

    const BB0: char = '='; // GameScreen::BB0
    const BB1: char = '\\';// GameScreen::BB1
    const BB2: char = '/'; // GameScreen::BB2
    const BB3: char = ' '; // GameScreen::BB3


//let first_design: [[char; 24]; 22] = [
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
//     ['<','!','=','=','=','=','=','=','=','=','=','=','=','=','=','=','=','=','=','=','=','=','!','>'],
// [' ',' ','\\','/','\\','/','\\','/','\\','/','\\','/','\\','/','\\','/','\\','/','\\','/','\\','/',' ',' ']
//         ];

    pub fn new() -> Self{
    //[' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        let configured_layers: [[char; GameScreen::screen_width]; GameScreen::screen_height] = [
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::BC0,GameScreen::BC1,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::B0L,GameScreen::B1M,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::BB0,GameScreen::B1M,GameScreen::B3R],
    [GameScreen::BB3,GameScreen::BB3,GameScreen::BB1,GameScreen::BB2,GameScreen::BB1,GameScreen::BB2,GameScreen::BB1,GameScreen::BB2,GameScreen::BB1,GameScreen::BB2,GameScreen::BB1,GameScreen::BB2,GameScreen::BB1,GameScreen::BB2,GameScreen::BB1,GameScreen::BB2,GameScreen::BB1,GameScreen::BB2,GameScreen::BB1,GameScreen::BB2,GameScreen::BB1,GameScreen::BB2,GameScreen::BB3,GameScreen::BB3]
        ];
        GameScreen{
            game_board: [[false;GameScreen::logic_width];GameScreen::logic_height],
            rendered_board  : configured_layers,
            pieces          : from_fn(|_| Piece{
                p_type: Configuration::BB,
                config: Piece::BB,
                rotation: 0,
            }),
        }
    }
    fn generate_piece() -> Configuration{        
        match rand::rng().random_range(1..=4)
        {
            1 => {
                Configuration::RL
            },
            2 => {
                Configuration::LL
            },
            3 => {
                Configuration::TT
            },
            4 => {
                Configuration::SL
            },
            5 => {
                Configuration::SS
            },
            _=>{
                panic!("Configuration Generator Syntax Error") // Critical failure
            }
        }
    }
    fn display_board(&self)
    {
        
        for layer in self.rendered_board{
            for chars in layer{
                #[cfg(feature = "std")]
                print!("{}",chars);
            }
            #[cfg(feature = "std")]
            print!("\n");
        }
        #[cfg(feature = "std")]
        print!("\n");
    }
    fn update_render(&mut self)
    {   
        assert!(self.rendered_board[0].len()-4 == self.game_board[0].len()*2, "{}", format!("Update renderer: Width mismatch\n[{}]:[{}]",self.rendered_board[0].len(),self.game_board[0].len()));
        assert!(self.rendered_board.len()-2 == self.game_board.len()      , "{}", format!("Update renderer: Height mismatch\n[{}][{}]",self.rendered_board.len(), self.game_board.len()));


        self.game_board = [[true; GameScreen::logic_width]; GameScreen::logic_height]; //##############################


        for (ii, layer) in self.game_board.iter().enumerate(){
            for (jj, tile) in layer.iter().enumerate(){
                if *tile{
                    self.rendered_board[ii][2+jj*2]   = '[';
                    self.rendered_board[ii][2+jj*2+1] = ']';
                }
            }
        }
    }

}
fn main() {
    let mut screen = GameScreen::new();
    screen.display_board();
    screen.update_render();
    screen.display_board();
}
