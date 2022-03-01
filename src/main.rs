use clap::{Parser, Subcommand};
extern crate termion;
use std::io::{stdin, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod menu;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Update the remote settings
    #[clap(long)]
    upload: bool,

    /// Update the local settings
    #[clap(long)]
    download: bool,
}

fn main() -> Result<(), std::io::Error> {
    menu::test();

    let cli = Cli::parse();

    if cli.upload {
        println!("Uploading");
        return Ok(());
    }
    if cli.download {
        println!("Downloading");
        return Ok(());
    }

    interface()
}

fn interface() -> Result<(), std::io::Error> {
    let stdin = stdin();
    let mut stdout = std::io::stdout().into_raw_mode()?;

    let height: u16 = termion::terminal_size()?.1;

    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();


    let mut x = 1;
    let mut y = 2;

    
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    Ok(())
}

