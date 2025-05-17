mod tetris;
mod terminal;
fn main() {
    terminal::test();
}

fn tetris_wrapper(){
    
    let mut screen = tetris::GameScreen::new();
    screen.display_board();
    screen.update_render();
    screen.display_board();

}