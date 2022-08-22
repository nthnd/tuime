    use termion::cursor;
    use std::io::{Write, Stdout};
    pub fn draw(stdout : &mut Stdout, num: u16, pos_x: u16,mut pos_y: u16){
        let  ascii_num: [u16; 15] ;
        match num {
            1 => {
                ascii_num = [
                    0,1,0, 
                    1,1,0, 
                    0,1,0, 
                    0,1,0, 
                    1,1,1,
                ]
            },

            2 => {
                ascii_num = [
                    1,1,1, 
                    0,0,1, 
                    1,1,1, 
                    1,0,0, 
                    1,1,1,
                ]
            },

            3 => {
                ascii_num = [
                    1,1,1, 
                    0,0,1, 
                    0,1,1, 
                    0,0,1, 
                    1,1,1,
                ]
            },

            4 => {
                ascii_num = [
                    1,0,1, 
                    1,0,1, 
                    1,1,1, 
                    0,0,1, 
                    0,0,1,
                ]
            },

            5 => {
                ascii_num = [
                    1,1,1, 
                    1,0,0, 
                    1,1,1, 
                    0,0,1, 
                    1,1,1,
                ]
            },

            6 => {
                ascii_num = [
                    1,0,0, 
                    1,0,0, 
                    1,1,1, 
                    1,0,1, 
                    1,1,1,
                ]
            },

            7 => {
                ascii_num = [
                    1,1,1, 
                    0,0,1, 
                    0,0,1, 
                    0,0,1, 
                    0,0,1,
                ]
            },

            8 => {
                ascii_num = [
                    1,1,1, 
                    1,0,1, 
                    1,1,1, 
                    1,0,1, 
                    1,1,1,
                ]
            },

            9 => {
                ascii_num = [
                    1,1,1, 
                    1,0,1, 
                    1,1,1, 
                    0,0,1, 
                    0,0,1,
                ]
            },
            0 => {
                ascii_num = [
                    1,1,1,
                    1,0,1,
                    1,0,1,
                    1,0,1,
                    1,1,1,
                ]
            }
            _ => {
                ascii_num = [
                    0,0,0,
                    0,1,0,
                    0,0,0,
                    0,1,0,
                    0,0,0,
                ]
            }
        }
        for i in 0..15{
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
                if ascii_num[i] == 0 {"  "} else {"██"},
            ).unwrap();
        }
    }
