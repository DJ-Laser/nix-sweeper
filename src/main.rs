use std::io::{self, Write, stdout};

use crossterm::{
    cursor,
    event::{self, KeyCode, KeyModifiers},
    terminal,
};
use output::{print_formatted, switch_terminal_mode};

mod ffi;
mod output;
mod random;

fn event_loop(stdout: &mut impl Write) -> io::Result<()> {
    let state = ffi::initial(10, 10, 20)?;

    loop {
        crossterm::execute!(
            stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::All),
            terminal::Clear(terminal::ClearType::Purge),
            print_formatted(&ffi::output(&state)?)
        )?;

        match event::read()? {
            event::Event::Key(key_event) => match key_event.code {
                KeyCode::Char('c') if matches!(key_event.modifiers, KeyModifiers::CONTROL) => break,
                KeyCode::Char('q') => break,

                _ => (),
            },
            _ => (),
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    switch_terminal_mode(&mut stdout, true)?;

    if let Err(error) = event_loop(&mut stdout) {
        switch_terminal_mode(&mut stdout, false)?;
        return Err(io::Error::new(
            error.kind(),
            format!("Unexpected error: {:?}", error),
        ));
    }

    switch_terminal_mode(&mut stdout, false)?;
    Ok(())
}
