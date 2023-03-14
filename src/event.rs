
use termion::event::{Event, Key};

use crate::state::{State};

pub struct EventHandler {

}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
        }
    }

    pub fn handle_events(&self, event: Event, state: &mut State) {
        match event {
            Event::Key(k) => {
                match k {
                    Key::Char('q') => { state.should_quit = true; },
                    Key::Char('j') => { state.selected_option.toggle(); },
                    Key::Up => { state.selected_option.toggle(); },
                    Key::Char('k') => { state.selected_option.toggle(); },
                    Key::Down => { state.selected_option.toggle(); },


                    Key::Char('l')  => { state.selected_option.locked = true },
                    Key::Right => { state.selected_option.locked = false },
                    Key::Char('h')  => { state.selected_option.locked = false },
                    Key::Left => { state.selected_option.locked = false },

                    Key::Char('0') => { state.selected_number.push('0'); },
                    Key::Char('1') => { state.selected_number.push('1'); },
                    Key::Char('2') => { state.selected_number.push('2'); },
                    Key::Char('3') => { state.selected_number.push('3'); },
                    Key::Char('4') => { state.selected_number.push('4'); },
                    Key::Char('5') => { state.selected_number.push('5'); },
                    Key::Char('6') => { state.selected_number.push('6'); },
                    Key::Char('7') => { state.selected_number.push('7'); },
                    Key::Char('8') => { state.selected_number.push('8'); },
                    Key::Char('9') => { state.selected_number.push('9'); },
                    Key::Backspace => { state.selected_number.pop(); },
                    Key::Delete => { state.selected_number.pop(); },
                    _ => {}
                }
            },
            Event::Mouse(_) => {},
            _ => {}
        }
    }
}