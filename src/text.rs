    use termion::{color, cursor };
    use std::io::{Write, Stdout};
    pub fn draw(stdout : &mut Stdout, num: u16, pos_x: u16,mut pos_y: u16){
        let  ascii_num: [u16; 15] =
        match num {
            1 => {
               [
                    0,1,0, 
                    1,1,0, 
                    0,1,0, 
                    0,1,0, 
                    1,1,1,
                ]
            }
            2 => {
               [
                    1,1,1, 
                    0,0,1, 
                    1,1,1, 
                    1,0,0, 
                    1,1,1,
                ]
            }
            3 => {
               [
                    1,1,1, 
                    0,0,1, 
                    0,1,1, 
                    0,0,1, 
                    1,1,1,
                ]
            }
            4 => {
               [
                    1,0,1, 
                    1,0,1, 
                    1,1,1, 
                    0,0,1, 
                    0,0,1,
                ]
            }
            5 => {
               [
                    1,1,1, 
                    1,0,0, 
                    1,1,1, 
                    0,0,1, 
                    1,1,1,
                ]
            }
            6 => {
               [
                    1,0,0, 
                    1,0,0, 
                    1,1,1, 
                    1,0,1, 
                    1,1,1,
                ]
            }
            7 => {
               [
                    1,1,1, 
                    0,0,1, 
                    0,0,1, 
                    0,0,1, 
                    0,0,1,
                ]
            }
            8 => {
               [
                    1,1,1, 
                    1,0,1, 
                    1,1,1, 
                    1,0,1, 
                    1,1,1,
                ]
            }
            9 => {
               [
                    1,1,1, 
                    1,0,1, 
                    1,1,1, 
                    0,0,1, 
                    0,0,1,
                ]
            }
            0 => {
               [
                    1,1,1,
                    1,0,1,
                    1,0,1,
                    1,0,1,
                    1,1,1,
                ]
            }
            _ => {
               [
                    0,0,0,
                    0,1,0,
                    0,0,0,
                    0,1,0,
                    0,0,0,
                ]
            }
        };
        for (i, &num) in ascii_num.iter().enumerate(){
            if i %3 == 0 {
                pos_y += 1;
                write!(
                    stdout, "{}",
                    cursor::Goto(pos_x , pos_y),
                ).unwrap();
            }
            write!(
                stdout,
                "{}",
                if num == 0 {"  "} else {"██"},
            ).unwrap();
        }
    }
pub fn apply_color(key: u32, stdout: &mut Stdout) {
    match key {
        1 => {
            write!(stdout, "{}", color::Fg(color::LightBlack)).unwrap();
        }
        2 => {
            write!(stdout, "{}", color::Fg(color::LightRed)).unwrap();
        }
        3 => {
            write!(stdout, "{}", color::Fg(color::LightGreen)).unwrap();
        }
        4 => {
            write!(stdout, "{}", color::Fg(color::LightYellow)).unwrap();
        }
        5 => {
            write!(stdout, "{}", color::Fg(color::LightBlue)).unwrap();
        }
        6 => {
            write!(stdout, "{}", color::Fg(color::LightMagenta)).unwrap();
        }
        7 => {
            write!(stdout, "{}", color::Fg(color::LightCyan)).unwrap();
        }
        8 => {
            write!(stdout, "{}", color::Fg(color::LightWhite)).unwrap();
        }
        _ => {
           write!(stdout, "{}", color::Fg(color::Reset)).unwrap();
        }
    }
}
