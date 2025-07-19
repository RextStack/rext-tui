use rext_tui::{App, error::RextTuiError};

fn main() -> Result<(), RextTuiError> {
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
