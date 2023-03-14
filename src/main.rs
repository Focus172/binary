
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
use event::EventHandler;

fn main() -> Result<(), Error> {

    let screen: RawTerminal<AlternateScreen<Stdout>> = stdout().into_alternate_screen()?.into_raw_mode()?;
    //let mouse_screen = MouseTerminal::from(stdout().into_alternate_screen()?.into_raw_mode()?);
    let mut state =  State::new();
    let mut renderer = Renderer::new(screen);
    let event_handler = EventHandler::new();

    renderer.inititial_message()?;

    let stdin = stdin();
    for evt in stdin.events() {
        if let Ok(evt) = evt {
            event_handler.handle_events(evt, &mut state);
            renderer.render(&state)?;
        }
        if state.should_quit { break; }
    }

    println!("Done!");
    
    /* 
    

    write!(stdout, "{}{}q to exit. Click, click, click!", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
                    },
                    _ => (),
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
    */

    Ok(())

    /*
    let stdin = io::stdin();
    //let thing = stdin.lock();
    
    let input = Input::new(stdin);
    let event_stream = input.handle_keydowns();


    let mut state = TermState::new();
    let mut renderer = Renderer::new();
    let mut keys = KeyState::new(); 

    //let mut term: Box<dyn Terminal<Output = io::Stdout>> = term::stdout().unwrap();
    
    let mut should_render = false;
    
    thread::sleep(Duration::from_millis(1000));

    loop {

        for received in &event_stream {
            match received {
                KeyDown::Up => keys.up_pressed = true,
                KeyDown::Down => keys.down_pressed = true,
                KeyDown::None => {
                    keys.up_pressed = false;
                    keys.down_pressed = false;
                }
            }
            println!("{:?}", received);
        }

        state.parse_input(&keys, &mut should_render);

        
        //if should_render {
            //renderer.render(&state, &mut term);
        //}

        thread::sleep(Duration::from_millis(1000));

    }
    */
    //thread::sleep(Duration::from_millis(300));

    /*
    let mut res: Option<i64> = Option::None;

    while res.is_none() {
        println!("Welcome to a this program that convers things");
        println!("Choose an options:");
        println!("1. Convert from Binary to Decimal");
        println!("2. Convert from Decimal to Binary");
        print!("Enter your choice: "); std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let _ = std::io::stdin().read_line(&mut buffer);

        //checks if the input is a number
        let num = buffer.trim().parse::<i64>();

        match num {
            Ok(number) => res = Some(number),
            Err(_) => {
                println!("Invalid choice")
            }
        }
    }

    let num = res.unwrap();

    match num {
        1 => to_binary(),
        2 => to_decimal(),
        _ => {
            println!("Invalid choice");
            main();
        },
    };
    */
}
