//! Main entry point of the project.

use crate::tui::App;
mod unicode;
mod tui;

fn main() -> anyhow::Result<()>{
    let terminal = ratatui::init();
    App::new()?.run(terminal)?;
    ratatui::restore();
    Ok(())
}