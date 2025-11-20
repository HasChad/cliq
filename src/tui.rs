use crossterm::{
    cursor::{MoveRight, MoveTo, MoveToNextLine, Show},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{EnableLineWrap, SetSize},
};
use std::io::{self, Stdout, Write};

use crate::{App, Popup, popups::*};

pub fn render(stdout: &mut Stdout, app: &mut App) -> io::Result<()> {
    draw_box_with_title(
        stdout,
        app.size.0,
        app.size.1 - 5,
        0,
        0,
        "Chat".into(),
        Color::DarkYellow,
    )?;

    draw_box_with_title(
        stdout,
        app.size.0,
        5,
        0,
        app.size.1 - 5,
        "Message".into(),
        Color::White,
    )?;

    queue!(
        stdout,
        MoveTo(1, 1),
        EnableLineWrap,
        SetSize(app.size.0 - 2, app.size.1 - 2)
    )?;

    for message in app.messages.iter() {
        if message.role == "user" || message.role == "system" {
            queue!(
                stdout,
                SetForegroundColor(Color::Blue),
                Print("You: "),
                SetForegroundColor(Color::Reset),
            )?;
        } else if message.role == "assistant" {
            queue!(
                stdout,
                SetForegroundColor(Color::DarkRed),
                Print("AI: "),
                SetForegroundColor(Color::Reset),
            )?;
        }

        let wrapped_text = textwrap::wrap(&message.content, app.size.0 as usize - 7);

        for text in wrapped_text.iter() {
            queue!(stdout, Print(text), MoveToNextLine(1), MoveRight(1))?;
        }
        queue!(stdout, MoveToNextLine(1), MoveRight(1))?;
    }

    match &app.popup {
        Popup::Welcome => popup_welcome(stdout, &app.size)?,
        Popup::Help => popup_help(stdout, &app.size)?,
        Popup::Status => popup_status(stdout, &app.size, &app.messages)?,
        Popup::Error(msg) => popup_error(stdout, &app.size, msg.as_str())?,
        Popup::None => {
            let wrap_text = textwrap::wrap(app.input.as_str(), app.size.0 as usize - 11);

            queue!(
                stdout,
                MoveTo(1, app.size.1 - 4),
                SetForegroundColor(Color::Blue),
                Print("Message: "),
                SetForegroundColor(Color::Reset),
            )?;

            for (i, text) in wrap_text.iter().enumerate() {
                queue!(stdout, Show, Print(text))?;

                if i + 1 != wrap_text.len() {
                    queue!(stdout, MoveToNextLine(1), MoveRight(1))?;
                }
            }
        }
    }

    stdout.flush()?;

    Ok(())
}

pub fn draw_box(
    stdout: &mut Stdout,
    width: u16,
    height: u16,
    x_pos: u16,
    y_pos: u16,
    color: Color,
) -> io::Result<()> {
    queue!(stdout, SetForegroundColor(color))?;

    queue!(
        stdout,
        MoveTo(x_pos, y_pos),
        Print("╭"),
        Print("─".repeat((width - 2) as usize)),
        Print("╮")
    )?;

    for y in 1..height - 1 {
        queue!(
            stdout,
            MoveTo(x_pos, y_pos + y),
            Print("│"),
            Print(" ".repeat((width - 2) as usize)),
            Print("│")
        )?;
    }

    queue!(
        stdout,
        MoveTo(x_pos, y_pos + height - 1),
        Print("╰"),
        Print("─".repeat((width - 2) as usize)),
        Print("╯")
    )?;

    queue!(stdout, SetForegroundColor(Color::Reset))?;
    Ok(())
}

pub fn draw_box_with_title(
    stdout: &mut Stdout,
    width: u16,
    height: u16,
    x_pos: u16,
    y_pos: u16,
    title: String,
    color: Color,
) -> io::Result<()> {
    draw_box(stdout, width, height, x_pos, y_pos, color)?;

    queue!(
        stdout,
        SetForegroundColor(color),
        MoveTo(x_pos + 3, y_pos),
        Print(format!(" {} ", title)),
        SetForegroundColor(Color::Reset)
    )?;

    Ok(())
}

pub fn screen_size_warning(stdout: &mut Stdout, size: &(u16, u16)) -> io::Result<()> {
    let raw_text = format!(
        "Terminal size is too low! Width: {}, Height: {}
Set your terminal size to minimum Width: 80, Height: 20",
        size.0, size.1
    );
    let wrap_text = textwrap::wrap(raw_text.as_str(), size.0 as usize);

    let mut y_pos = size.1 / 2 - 1;
    execute!(stdout, MoveTo(0, y_pos - 1))?;

    for text in wrap_text.iter() {
        let x_pos = (size.0 - text.len() as u16) / 2;
        y_pos += 1;
        execute!(stdout, MoveTo(x_pos, y_pos), Print(text))?;
    }

    Ok(())
}
