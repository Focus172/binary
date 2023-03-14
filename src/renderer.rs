

use crate::state::{State, OptionState};
use std::io::{Write, Stdout, Error};
use termion::{raw::RawTerminal, screen::AlternateScreen, color};
use crate::logic::{to_decimal, to_binary};

pub struct Renderer {
    screen: RawTerminal<AlternateScreen<Stdout>>,
}

impl Renderer {
    pub fn new(screen: RawTerminal<AlternateScreen<Stdout>>) -> Renderer {
        Renderer {
            screen: screen,
        }
    }

    pub fn inititial_message(&mut self) -> Result<(), Error>  {
        write!(self.screen, "{}", termion::cursor::Goto(1, 1))?;
            write!(self.screen, "{}{}", color::Bg(color::Reset), termion::clear::All)?;
            write!(self.screen, "{}Use (j,k) to move and (h,l) to navigate in and out of menus, press 'q' at anytime to quit.", color::Bg(color::Reset))?;

            write!(self.screen, "{}", termion::cursor::Goto(1, 2))?;
            write!(self.screen, "{}1) Binary to decimal", color::Bg(color::Red))?;
            write!(self.screen, "{}", termion::cursor::Goto(1, 3))?;
            write!(self.screen, "{}2) Decimal to binary", color::Bg(color::Reset))?;
            self.screen.flush()?;
            Ok(())
    }

    pub fn render(&mut self, state: &State) -> Result<(), Error> {
        if state.selected_option.locked {
            write!(self.screen, "{}{}", color::Bg(color::Reset), termion::clear::All)?;

            match state.selected_option.state {
                OptionState::BinaryToDecimal => {
                    write!(self.screen, "{}", termion::cursor::Goto(1, 1))?;
                    write!(self.screen, "Convert a number from binary to decimal! You can press 'h' to go back to select.")?;
                },
                OptionState::DecimalToBinary => {
                    write!(self.screen, "{}", termion::cursor::Goto(1, 1))?;
                    write!(self.screen, "Convert a number from decimal to binary! You can press 'h' to go back to select.")?;
                    
                }
            }

            write!(self.screen, "{}", termion::cursor::Goto(1, 2))?;
            write!(self.screen, "Enter: {}", state.selected_number.number)?;
            

            let result = match state.selected_option.state {
                OptionState::BinaryToDecimal => {
                    to_decimal(state.selected_number.number.as_str())
                },
                OptionState::DecimalToBinary => {
                    to_binary(state.selected_number.number.as_str())
                }
            };
            write!(self.screen, "{}", termion::cursor::Goto(1, 3))?;
            write!(self.screen, "{}", result)?;

            write!(self.screen, "{}", termion::cursor::Goto((8+state.selected_number.number.len()) as u16, 2))?;
            self.screen.flush()?;
            
        } else {
            write!(self.screen, "{}", termion::cursor::Goto(1, 1))?;
            write!(self.screen, "{}{}", color::Bg(color::Reset), termion::clear::All)?;
            write!(self.screen, "{}Use (j,k) to move and (h,l) to navigate in and out of menus, press 'q' at anytime to quit.", color::Bg(color::Reset))?;

            match state.selected_option.state {
                OptionState::DecimalToBinary => {
                    write!(self.screen, "{}", termion::cursor::Goto(1, 2))?;
                    write!(self.screen, "{}1) Binary to decimal", color::Bg(color::Reset))?;
                    write!(self.screen, "{}", termion::cursor::Goto(1, 3))?;
                    write!(self.screen, "{}2) Decimal to binary", color::Bg(color::Red))?;
                },
                OptionState::BinaryToDecimal => {
                    write!(self.screen, "{}", termion::cursor::Goto(1, 2))?;
                    write!(self.screen, "{}1) Binary to decimal", color::Bg(color::Red))?;
                    write!(self.screen, "{}", termion::cursor::Goto(1, 3))?;
                    write!(self.screen, "{}2) Decimal to binary", color::Bg(color::Reset))?;
                }
            }
            self.screen.flush()?;
        }

        Ok(())
    }
}

/*
print!("{}", DANCING_KIRBY[count % 8]);
        //print!("\n{} >>> {}", count, count+1);
        std::io::stdout().flush().unwrap();
        t.carriage_return().unwrap(); // get rid of standard newline after flush


        thread::sleep(Duration::from_millis(200));
        t.delete_line().unwrap();
        
        if count > 100 {
            break;
        }

        let x = std::io::stdout();
        x.lock().flush().unwrap();

        count += 1;
        */