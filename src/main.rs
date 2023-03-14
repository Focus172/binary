
extern crate termion;

mod renderer;
mod state;
mod event;
mod logic;

use termion::input::{TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{IntoAlternateScreen, AlternateScreen};
use std::io::{stdout, stdin, Error, Stdout};
use state::State;
use renderer::Renderer;

fn main() -> Result<(), Error> {
    
    // restore terminal on exit
    {
        let screen: RawTerminal<AlternateScreen<Stdout>> = stdout().into_alternate_screen()?.into_raw_mode()?;
        let mut state =  State::new();
        let mut renderer = Renderer::new(screen);

        renderer.inititial_message()?;

        let stdin = stdin();
        for evt in stdin.events() {
            if let Ok(evt) = evt {
                event::handle_events(evt, &mut state);
                renderer.render(&state)?;
            }
            if state.should_quit { break; }
        }
    }

    println!("Done!");

    Ok(())
}
