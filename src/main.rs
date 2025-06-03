//! Main entry point of the project.

use crate::tui::App;
mod unicode;
mod tui;

fn main() -> anyhow::Result<()>{
    let terminal = ratatui::init();
    App::default().run(terminal)?;
    ratatui::restore();
    Ok(())
}