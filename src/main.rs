use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
    QueueableCommand,
};
use std::{
    io::{self, Stdout, Write},
    time::Duration,
};

pub mod rect;

fn render(mut stdout: &Stdout, input: &[u8], content: Vec<String>) -> io::Result<()> {
    let (w, h) = terminal::size().unwrap();

    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    let text_input = rect::Rect {
        x: 2,
        y: h - 4,
        w: w - 4,
        h: 3,
    };

    let text_display = rect::Rect {
        x: 2,
        y: 1,
        w: w - 4,
        h: h - 5,
    };
    text_display.render_text(&stdout, &content.join("\n\r    ").as_bytes(), false, false);
    text_display.render(&stdout);

    text_input.render(&stdout);
    text_input.render_text(&stdout, &[b"|> ", input].concat(), false, true);

    stdout.flush()
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    let mut input = String::new();
    let mut content: Vec<String> = [].to_vec();

    execute!(stdout, EnableMouseCapture)?;
    loop {
        // Handle events
        if poll(Duration::from_millis(200))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Enter => {
                        if input.is_empty() {
                            continue;
                        }
                        content.push(input.to_string());
                        if content.len() as u16 > terminal::size().unwrap().1 - 7 {
                            content.remove(0);
                        }
                        input = String::new();
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Delete => {
                        content = [].to_vec();
                        input = String::new();
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    _ => {}
                }
            }
        }

        // Render screen
        render(&stdout, input.as_bytes(), content.clone())?;
    }

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()?;
    Ok(())
}
