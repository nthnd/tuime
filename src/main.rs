use std::io::{ Write,stdout};
use termion::{screen::*,raw::IntoRawMode,cursor,event::*, input::TermRead};
use std::thread;
use std::time::Duration;
mod text;
fn main() {
    //initialize and create an alternate screen
    let mut input_events = termion::async_stdin().events();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut quit = false;
    write!(stdout, "{}", cursor::Hide).unwrap();

    //main loop
    while !quit {

        //tick
        thread::sleep(Duration::from_secs(1));
        let current_time = chrono::offset::Local::now();

        //get current time
        let formatted = format!("{}", current_time.format("%H:%M"));
        let (center_x, center_y) = termion::terminal_size().unwrap();
        let (center_x, center_y) = (center_x/2, center_y/2) ;

        //draw current time
        write!(stdout, "{}", termion::clear::All).unwrap();
        for (index,character) in formatted.chars().enumerate(){
            let char_to_write = character.to_digit(10).unwrap_or(10) as u16;
            text::draw(
                &mut stdout,
                char_to_write,
                7 + (center_x as isize +  7 * (( index as isize - formatted.len() as  isize )  + (formatted.len()/2 )as isize)) as u16,
                center_y - 2
            );
        }
        stdout.flush().unwrap();
        
        //read input events and quit if q is pressed
        for input_event in &mut input_events{
            match input_event{
                Ok(e)=>{
                    if let Event::Key(key) = e{
                        match key {
                            Key::Char('q') => {
                                quit = true;
                                break;
                            }
                            Key::Char(x) => {
                                if x.is_ascii_digit(){
                                    text::apply_color(x.to_digit(10).unwrap(), &mut stdout);
                                }
                            }
                            _ => {}
                        }
                    }
                },
                Err(_e) => {
                    panic!("event_error");
                }
            }
        }
    }
    
    //back to main screen
    write!(stdout, "{}{}",ToMainScreen, cursor::Show).unwrap();
    stdout.flush().unwrap();
}
