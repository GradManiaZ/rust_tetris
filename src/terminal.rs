use std::{
  fmt::write, fs::read, io::{stdout, Write}, os::windows::{io::AsRawHandle, thread}, thread::sleep, time::Duration
};
use crossterm::{
  cursor, event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, ModifierKeyCode}, execute, terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, QueueableCommand
};



pub fn test(){

  print_events();
}
//key_map:HashSet<char>
pub fn print_events(){
  // key_map.get(value)
  let _ = execute!(std::io::stdout(), EnterAlternateScreen);
  let mut stdout = stdout();
  let _ = enable_raw_mode();
  loop {
      if poll(Duration::ZERO).unwrap(){
        match crossterm::event::read()
        {
          Ok(event) => {
           match event {
            Event::Key(key_event) 
            if key_event.kind == KeyEventKind::Press=>{
              match key_event.code
              {
                KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL
                =>{
                  let (buf_char, count) = ("#",10);
                  let buf = format!("\n{}Good Bye!{}\n",buf_char.repeat(count),buf_char.repeat(count));
                  stdout.write(buf.as_bytes()).unwrap();
                  sleep(Duration::from_millis(666));
                  break;
                }
                KeyCode::Char(x)=>
                {
                  stdout.write(&[x as u8]).unwrap();
                }
                _=>{

                }
              }
            },
            _ => {

            }
           }
          },
          Ok(ev) =>
          {
            let msg = format!("Unexptected Command: {:?}",ev);
            dbg!()
          },
          Err(e) => {
            panic!("Unable to read Terminal! {e}")
          }
        }
        stdout.flush();
      }
      sleep(Duration::from_millis(16));
  }
  let _ = disable_raw_mode();
  let _ = execute!(std::io::stdout(), LeaveAlternateScreen);

}