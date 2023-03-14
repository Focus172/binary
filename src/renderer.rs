

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
        render_select_screen(&mut self.screen, 1)?;
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
            match state.selected_option.state {
                OptionState::DecimalToBinary => { render_select_screen(&mut self.screen, 1)?; },
                OptionState::BinaryToDecimal => { render_select_screen(&mut self.screen,2)?; },
            }
            self.screen.flush()?;
        }

        Ok(())
    }
}

fn render_select_screen(screen: &mut RawTerminal<AlternateScreen<Stdout>>, selected_number: i8) -> Result<(), Error> {
    write!(screen, "{}{}{}Use (j,k) to move and (h,l) to navigate in and out of menus, press 'q' at anytime to quit.",
        termion::cursor::Goto(1, 1),
        color::Bg(color::Reset),
        termion::clear::All)?;

    if selected_number == 1 {
        write!(screen, "{}", termion::cursor::Goto(1, 2))?;
        write!(screen, "{}1) Binary to decimal", color::Bg(color::Red))?;
        write!(screen, "{}", termion::cursor::Goto(1, 3))?;
        write!(screen, "{}2) Decimal to binary", color::Bg(color::Reset))?;
    } else if selected_number == 2 {
        write!(screen, "{}", termion::cursor::Goto(1, 2))?;
        write!(screen, "{}1) Binary to decimal", color::Bg(color::Reset))?;
        write!(screen, "{}", termion::cursor::Goto(1, 3))?;
        write!(screen, "{}2) Decimal to binary", color::Bg(color::Red))?;
    } else {
        panic!();
    }

    Ok(())
}