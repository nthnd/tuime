use std::io::{ Write,stdout};
use termion::{screen::*,raw::IntoRawMode,cursor,event::*, input::TermRead};
use chrono;
use std::thread;
use std::time::Duration;
mod text;
fn main() {
    let mut input_events = termion::async_stdin().events();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut quit = false;
    write!(stdout, "{}", cursor::Hide).unwrap();
    while !quit {
        thread::sleep(Duration::from_secs(1));
        let current_time = chrono::offset::Local::now();
        let formatted = format!("{}", current_time.format("%H:%M:%S"));
        let (center_x, center_y) = termion::terminal_size().unwrap();
        let (center_x, center_y) = (center_x/2, center_y/2) ;
        write!(stdout, "{}", termion::clear::All).unwrap();
        for (i,c) in formatted.chars().enumerate(){
            if c.is_numeric() {
                text::draw(
                    &mut stdout,
                    c.to_digit(10).unwrap().try_into().unwrap(),
                    (center_x as isize +  4 * ((( i as isize - formatted.len() as  isize )  + (formatted.len()/2 )as isize))) as u16,
                    center_y - 2
                          );
            }else{
                text::draw(
                    &mut stdout,
                    10,
                    (center_x as isize +  4 * ((( i as isize - formatted.len() as  isize )  + (formatted.len()/2 )as isize))) as u16,
                    center_y - 2
                          );
            }
        }
        stdout.flush().unwrap();

        for input_event in &mut input_events{
            match input_event{
                Ok(e)=>{
                    match e{
                        Event::Key(key) => {
                            match key {
                                Key::Char('q') => {
                                    quit = true;
                                    break;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                },
                Err(_e) => {
                    panic!("event_error");
                }
            }
        }
    }
    write!(stdout, "{}{}{}",termion::clear::All,ToMainScreen, cursor::Show).unwrap();
    stdout.flush().unwrap();
}
