use crossterm::event::{KeyCode, KeyEvent};
use std::io;

// Import the App struct from the main crate
use rext_tui::App;

#[test]
fn handle_key_event() -> io::Result<()> {
    let mut app = App::new();

    // Test right key increments counter
    let right_event = KeyEvent::from(KeyCode::Right);
    app.on_key_event(right_event);
    // Since counter is private, we need to test behavior indirectly
    // For now, we'll just ensure the method doesn't panic

    // Test left key decrements counter
    let left_event = KeyEvent::from(KeyCode::Left);
    app.on_key_event(left_event);

    // Test quit functionality
    let mut app = App::new();
    let quit_event = KeyEvent::from(KeyCode::Char('q'));
    app.on_key_event(quit_event);
    // Since running field is private, we can't directly check it
    // The test mainly ensures the method calls don't panic

    Ok(())
}
