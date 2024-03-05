// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! A bare-bones terminal app that displays an all-white screen. Intended to be run on a secondary
//! monitor (or two) during video calls as a cheap key/fill light.
//!
//! Tested on a few Linux and Windows terminals and should work on any terminal/OS combination [supported by crossterm](https://github.com/crossterm-rs/crossterm#tested-terminals).
//!
//! # Alternatives
//! * Open about:blank or any predominantly-white page in your browser
//! * Buy a real lighting setup
//!
//! # Disclaimer
//!
//! This is not an officially supported Google product

use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event},
    style::{Color, Colors, SetColors},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, Result, Write};

/// Enable light by enabling a white background in an alternate terminal window
fn enable_lighting() -> Result<()> {
    io::stdout()
        .execute(terminal::EnterAlternateScreen)?
        .execute(SetColors(Colors {
            background: Some(Color::White),
            foreground: Some(Color::Black),
        }))?
        .execute(Clear(ClearType::All))?
        .execute(cursor::Hide)?;
    // Enabling raw mode allows text input to trigger key events rather than being piped to stdout
    terminal::enable_raw_mode()?;
    Ok(())
}

/// Disable light by restoring main screen
fn disable_lighting() -> Result<()> {
    terminal::disable_raw_mode()?;
    io::stdout()
        .execute(cursor::Show)?
        .execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}

/// Print exit instructions at the bottom of the terminal
fn print_instructions() -> Result<()> {
    let (_, rows) = terminal::size()?;
    io::stdout().execute(MoveTo(0, rows))?;
    // Crossterm doc recommends using write! over println! in raw mode because newlines aren't
    // handled, but excluding the newline character seems to prevent the message from being printed.
    // Using writeln! here as a working compromise but not really sure what's best practice here.
    writeln!(io::stdout(), "Press any key to exit").ok();
    Ok(())
}

fn main() -> Result<()> {
    enable_lighting()?;
    print_instructions()?;
    loop {
        match event::read()? {
            Event::Resize(..) => {
                io::stdout().execute(Clear(ClearType::All))?;
                print_instructions()?;
            }
            Event::Key(_) | Event::Paste(_) => break,
            Event::Mouse(..) | Event::FocusGained | Event::FocusLost => (),
        }
    }
    disable_lighting()?;
    Ok(())
}
