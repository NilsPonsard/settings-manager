use clap::Parser;
extern crate termion;
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

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
    let menu = menu::Menu::new(
        "Test".to_string(),
        vec!["aa".to_string(), "bb".to_string(), "cc".to_string()],
        60,
        format!("{}", termion::color::Bg(termion::color::Blue)),
        format!("{}", termion::color::Bg(termion::color::LightBlack)),
        format!("{}", termion::color::Fg(termion::color::White)),
        format!("{}", termion::color::Bg(termion::color::Black)),
        format!("{}", termion::color::Fg(termion::color::White)),
    );

    let res = menu.ask()?;

    let menu2 = menu::Menu::new_default_style(
        "salut".to_string(),
        vec!["aa".to_string(), "bb".to_string(), "cc".to_string()],
    );

    menu2.ask()?;

    println!("");

    print!("{}", res);
    Ok(())
}
