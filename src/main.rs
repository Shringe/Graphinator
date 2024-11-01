mod tui;

use std::io;
use tui::Tui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let tui_result = Tui::default().run(&mut terminal);
    ratatui::restore();
    tui_result
}
