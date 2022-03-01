use std::{
    cmp::min,
    io::{Stdout, Write},
};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Menu {
    title: String,
    items: Vec<String>,
    desired_width: u16,
    background_color: String,
    foreground_color: String,
    menu_background_color: String,
    selected_background_color: String,
    selected_foreground_color: String,
}

impl Menu {
    pub fn new(
        title: String,
        items: Vec<String>,
        desired_width: u16,
        background_color: String,
        menu_background_color: String,
        foreground_color: String,
        selected_background_color: String,
        selected_foreground_color: String,
    ) -> Self {
        Menu {
            title,
            items,
            desired_width,
            background_color,
            foreground_color,
            menu_background_color,
            selected_background_color,
            selected_foreground_color,
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
            selected_background_color: "".to_string(),
            selected_foreground_color: "".to_string(),
        }
    }
    fn print(
        &self,
        stdout: &mut termion::raw::RawTerminal<Stdout>,
        selected: usize,
    ) -> Result<(), std::io::Error> {
        write!(
            stdout,
            "{}{}{}",
            &self.foreground_color,
            self.background_color,
            termion::clear::All,
        )
        .unwrap();

        self.print_menu(selected as u16, stdout, &self.items)?;
        stdout.flush().unwrap();

        Ok(())
    }

    pub fn get_width(&self) -> u16 {
        let max_width = termion::terminal_size().unwrap_or((0, 0)).0;

        min(self.desired_width, max_width)
    }

    pub fn ask(&self) -> Result<usize, std::io::Error> {
        // todo : fall back if the terminal dosen’t support the required capabilities.

        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout().into_raw_mode()?;

        let size = termion::terminal_size()?;

        let mut selected = 0;

        let max_width = size.0;
        let max_height = size.1;

        let window_height = min(self.items.len() as u16 + 3, max_height);

        write!(
            stdout,
            "{}{}{}{}",
            termion::clear::All,
            termion::color::Fg(termion::color::Reset),
            termion::color::Bg(termion::color::Reset),
            termion::cursor::Hide
        )?;

        self.print(&mut stdout, selected)?;

        for c in stdin.keys() {
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
                _ => {}
            }
            self.print(&mut stdout, selected)?;
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

    fn print_menu(
        &self,
        y: u16,
        stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
        options: &Vec<String>,
    ) -> Result<(), std::io::Error> {
        let mut i = 0;

        let width = self.get_width();

        let terminal_width = termion::terminal_size().unwrap_or((0, 0)).0;

        let padding_right = (terminal_width - width) / 2;

        self.print_background(stdout)?;
        write!(
            stdout,
            "{}{}{}{}{}{}",
            termion::cursor::Goto(padding_right, 1),
            self.menu_background_color,
            self.foreground_color,
            termion::style::Bold,
            self.title,
            termion::style::Reset
        )?;

        for _ in options.iter() {
            self.print_menu_line(i + 3, stdout, y == i, &options, 3, padding_right)?;
            i += 1;
        }

        stdout.flush()?;

        Ok(())
    }

    fn print_menu_line(
        &self,
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

        let reset = format!("{}{}", self.menu_background_color, self.foreground_color);

        let color = match selected {
            true => format!(
                "{}{}",
                self.selected_background_color, self.selected_foreground_color
            ),
            false => reset.to_string(),
        };

        let opt = format!(
            " {}{}",
            options[(target_y - offset) as usize].to_string(),
            std::iter::repeat(" ")
                .take(self.get_width() as usize - options[(target_y - offset) as usize].len() - 1)
                .collect::<String>()
        );

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

    pub fn print_background(
        &self,
        stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    ) -> Result<(), std::io::Error> {
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::color::Fg(termion::color::Reset),
            termion::color::Bg(termion::color::Reset),
        )?;

        for i in 1..self.items.len() + 3 {
            let width = self.get_width();

            let terminal_width = termion::terminal_size().unwrap_or((0, 0)).0;

            let padding_right = (terminal_width - width) / 2;
            write!(
                stdout,
                "{}{}{}",
                termion::cursor::Goto(padding_right, i as u16),
                self.menu_background_color,
                std::iter::repeat(" ")
                    .take(width as usize)
                    .collect::<String>()
            )?;
        }

        Ok(())
    }
}
