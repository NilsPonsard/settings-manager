use std::{cmp::min, io::Write};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub fn test() {
    println!("aaaa");
}

pub struct Menu {
    title: String,
    items: Vec<String>,
    desired_width: u16,
    background_color: String,
    foreground_color: String,
    menu_background_color: String,
}

impl Menu {
    pub fn new(
        title: String,
        items: Vec<String>,
        desired_width: u16,
        background_color: String,
        menu_background_color: String,
        foreground_color: String,
    ) -> Self {
        Menu {
            title,
            items,
            desired_width,
            background_color,
            foreground_color,
            menu_background_color,
        }
    }
    pub fn default() -> Self {
        Menu {
            title: "".to_string(),
            items: vec!["".to_string()],
            desired_width: 0,
            background_color: "".to_string(),
            foreground_color: "".to_string(),
            menu_background_color: "".to_string(),
        }
    }

    pub fn ask(&self) -> Result<usize, std::io::Error> {
        // todo : fall back if the terminal dosen’t support the required capabilities.

        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout().into_raw_mode()?;

        let size = termion::terminal_size()?;

        let mut selected = 0;

        let max_width = size.0;
        let max_height = size.1;

        let window_width = min(self.desired_width, max_width);
        let window_height = min(self.items.len() as u16 + 3, max_height);

        for c in stdin.keys() {
            write!(
                stdout,
                "{}{}{}",
                termion::clear::All,
                termion::color::Fg(termion::color::Reset),
                termion::color::Bg(termion::color::Reset)
            )
            .unwrap();
            match c.unwrap() {
                Key::Char('\n') => return Ok(selected),

                Key::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                Key::Down => {
                    if selected < self.items.len() - 1 {
                        selected += 1;
                    }
                }
                Key::Backspace => println!("×"),
                _ => {}
            }
            print_menu(selected as u16, &mut stdout, &self.items)?;
            stdout.flush().unwrap();
        }

        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Hide,
            self.background_color
        )?;

        // reset cursor config
        write!(stdout, "{}", termion::cursor::Show)?;
        Ok(0)
    }
}

fn print_menu(
    y: u16,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    options: &Vec<String>,
) -> Result<(), std::io::Error> {
    let mut i = 0;

    for opt in options.iter() {
        print_menu_line(i + 2, stdout, y == i, &options, 2, 16)?;
        i += 1;
    }

    stdout.flush()?;

    Ok(())
}

fn print_menu_line(
    target_y: u16,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    selected: bool,
    options: &Vec<String>,
    offset: u16,
    x: u16,
) -> Result<(), std::io::Error> {
    let check = u16::checked_sub(target_y, offset);

    if check.is_none() {
        return Ok(());
    }

    let reset = format!(
        "{}{}",
        termion::color::Fg(termion::color::Reset),
        termion::color::Bg(termion::color::Reset)
    );

    let color = match selected {
        true => format!(
            "{}{}",
            termion::color::Fg(termion::color::White),
            termion::color::Bg(termion::color::Blue)
        ),
        false => reset.to_string(),
    };

    let opt = options[(target_y - offset) as usize].to_string();

    write!(
        stdout,
        "{}{}{}{}",
        termion::cursor::Goto(x, target_y),
        color,
        opt,
        reset
    )?;

    stdout.flush()?;

    Ok(())
}
