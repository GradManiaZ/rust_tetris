use rand::prelude::*;
use core::array::from_fn;
use std::char;

#[derive(Clone)]
enum Configuration{
    RL,
    LL,
    TT,
    SL,
    SS,
    BB
}
#[derive(Clone)]
struct Piece{
    p_type    :  Configuration,
    config    :  [[bool;4];4],
    rotation  :  i32
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



    const RL:[[[bool;4];4];4] = [[
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
    }
struct GameScreen{
    game_board      :  [[bool; 10]; 20],
    rendered_board  :  [[char;24];22],
    pieces          :  [Piece;51] // 10 * 20 tiles, 4 tiles per block max, 1 redundant
}
impl GameScreen {
    pub fn new() -> Self{
    //[' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        let init_layers: [[char; 24]; 22] = [
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','.',' ','!','>'],
    ['<','!','=','=','=','=','=','=','=','=','=','=','=','=','=','=','=','=','=','=','=','=','!','>'],
[' ',' ','\\','/','\\','/','\\','/','\\','/','\\','/','\\','/','\\','/','\\','/','\\','/','\\','/',' ',' ']
        ];
        GameScreen{
            game_board: [[false;10];20],
            rendered_board  : init_layers,
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
                print!("{}",chars);
            }
            print!("\n");
        }
        print!("\n");
    }
    fn update_render(&mut self)
    {   
        assert!(self.rendered_board[0].len()-4 == self.game_board[0].len()*2, "{}", format!("Update renderer: Width mismatch\n[{}]:[{}]",self.rendered_board[0].len(),self.game_board[0].len()));
        assert!(self.rendered_board.len()-2 == self.game_board.len()      , "{}", format!("Update renderer: Height mismatch\n[{}][{}]",self.rendered_board.len(), self.game_board.len()));


        self.game_board = [[true; 10]; 20]; //##############################


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
