use rand::prelude::*;
use core::array::from_fn;
use std::char;


enum Configuration{
    LL,
    JJ,
    TT,
    SS,
    ZZ,
    II,
    OO,
    BB
}

pub struct Piece{
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



    const JJ:[[[bool;4];4];4] = [[
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




    const LL:[[[bool;4];4];4] = [[
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




    const ZZ:[[[bool;4];4];2]= [[
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




    const SS:[[[bool;4];4];2]= [[
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




    const II:[[[bool;4];4];2] = [[
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




    const OO:[[[bool;4];4];1] = [[
        [false,false,false,false],
        [false,false,false,false],
        [true ,true ,false,false],
        [true ,true ,false,false]
    ]];



    pub fn get_configuration(config:Configuration, rotation:usize) -> [[bool;4];4]
    {
        assert!(rotation < 4, "Function: Get Configuration. Piece Rotation Parsing");
        match config{
            Configuration::LL => {
                Piece::LL[rotation]
            },
            Configuration::JJ => {
                Piece::JJ[rotation]
            },
            Configuration::TT => {
                Piece::TT[rotation]
            },
            Configuration::SS => {
                match rotation{
                    0|2 => {
                        Piece::SS[0]
                    },
                    1|3 => {
                        Piece::SS[1]
                    }
                    _ => {
                        panic!("Function: Get Configuration. Piece Rotation PaSSing [SS]")
                    }
                }
            },
            Configuration::ZZ => {
                match rotation{
                    0|2 => {
                        Piece::ZZ[0]
                    },
                    1|3 => {
                        Piece::ZZ[1]
                    }
                    _ => {
                        panic!("Function: Get Configuration. Piece Rotation PaSSing [ZZ]")
                    }
                }
            },
            Configuration::II => {
                match rotation{
                    0|2 => {
                        Piece::II[0]
                    },
                    1|3 => {
                        Piece::II[1]
                    }
                    _ => {
                        panic!("Function: Get Configuration. Piece Rotation PaSSing [II]")
                    }
                }
            },
            Configuration::OO => {
                match rotation{
                    0|1|2|3 => {
                        Piece::OO[0]
                    }
                    _ => {
                        panic!("Function: Get Configuration. Piece Rotation Parsing [OO]")
                    }
                }
            },
            Configuration::BB => {
                Piece::BB
            }
        }
    }   
}

 
pub struct GameScreen{
    game_board      :  [[bool; GameScreen::LOGIC_WIDTH]; GameScreen::LOGIC_LIMIT],
    rendered_board  :  [[char; GameScreen::SCREEN_WIDTH];GameScreen::SCREEN_HEIGHT],
    pieces          :  [Piece; GameScreen::PIECE_LIMIT] // GameScreen::LOGIC_WIDTH * 20 tiles, 4 tiles per block max, 1 redundant
}

impl GameScreen {

    //Screen Size
    const SCREEN_WIDTH  :  usize  =  24;
    const SCREEN_HEIGHT :  usize  =  22;


    //State Machine Size
    const LOGIC_WIDTH   :  usize  = (GameScreen::SCREEN_WIDTH  -4) /2;
    const LOGIC_LIMIT  :  usize  = (GameScreen::SCREEN_HEIGHT -2);
    const PIECE_LIMIT   :  usize  = (GameScreen::LOGIC_WIDTH * GameScreen::LOGIC_LIMIT) /4 +1; 
    

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
        let configured_layers: [[char; GameScreen::SCREEN_WIDTH]; GameScreen::SCREEN_HEIGHT] = [
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
            game_board: [[false;GameScreen::LOGIC_WIDTH];GameScreen::LOGIC_LIMIT],
            rendered_board  : configured_layers,
            pieces          : from_fn(|_| Piece{
                p_type: Configuration::BB,
                config: Piece::BB,
                rotation: 0,
            }),
        }
    }
/*
LL
JJ
TT
SS
ZZ
II
OO
BB
*/
    fn generate_piece() -> (Configuration,usize){  
        let rotation =       rand::rng().random_range(0..4) as usize;
        assert!(rotation < 4);
        match rand::rng().random_range(1..=7)
        {
            1 => {
                (Configuration::LL, rotation)
            },
            2 => {
                (Configuration::JJ, rotation)
            },
            3 => {
                (Configuration::TT, rotation)
            },
            4 => {
                (Configuration::II, rotation)
            },
            5 => {
                (Configuration::OO, rotation)
            },
            6 => {
                (Configuration::II, rotation)
            },
            7 => {
                (Configuration::OO, rotation)
            },
            _=>{
                panic!("Configuration Generator Syntax Error") // Critical failure
            }
        }
    }
    pub fn display_board(&self)
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
    pub fn update_render(&mut self)
    {   
        assert!(self.rendered_board[0].len()-4 == self.game_board[0].len()*2, "{}", format!("Update renderer: Width mismatch\n[{}]:[{}]",self.rendered_board[0].len(),self.game_board[0].len()));
        assert!(self.rendered_board.len()-2 == self.game_board.len()      , "{}", format!("Update renderer: Height mismatch\n[{}][{}]",self.rendered_board.len(), self.game_board.len()));


        self.game_board = [[true; GameScreen::LOGIC_WIDTH]; GameScreen::LOGIC_LIMIT]; //##############################


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

