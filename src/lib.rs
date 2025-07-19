pub mod config;
pub mod error;

use crate::config::load_config;
use crate::error::RextTuiError;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Is the API endpoint dialog open?
    pub dialog_open: bool,
    /// Text input buffer for API endpoint name
    pub api_endpoint_input: String,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), RextTuiError> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    fn render(&mut self, frame: &mut Frame) {
        // Load colors
        let (primary_color, text_color, background_color) = self.load_colors();

        // Set background color
        let background = Block::default().style(Style::default().bg(background_color));
        frame.render_widget(background, frame.area());

        // Main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Top area for button
                Constraint::Min(0),    // Rest of the screen
            ])
            .split(frame.area());

        // Top area with "add API endpoint" button
        let top_area = chunks[0];
        let button_text = Line::from(vec![
            Span::styled(
                "Add API endpoint",
                Style::default().fg(primary_color).bold(),
            ),
            Span::styled(" (e)", Style::default().fg(text_color)),
        ]);

        let button_paragraph = Paragraph::new(button_text).style(Style::default().fg(text_color));
        frame.render_widget(
            button_paragraph,
            Rect::new(top_area.x + 1, top_area.y + 1, top_area.width, 1),
        );

        // Bottom area with quit instructions
        let bottom_area = chunks[1];
        let quit_instructions = Line::from(vec![
            Span::styled("Press ", Style::default().fg(text_color)),
            Span::styled("q", Style::default().fg(primary_color).bold()),
            Span::styled(" or ", Style::default().fg(text_color)),
            Span::styled("Ctrl+C", Style::default().fg(primary_color).bold()),
            Span::styled(" to quit", Style::default().fg(text_color)),
        ]);

        let quit_paragraph = Paragraph::new(quit_instructions).alignment(Alignment::Center);

        // Position quit message at bottom of screen
        let quit_rect = Rect::new(
            bottom_area.x,
            bottom_area.y + bottom_area.height - 2,
            bottom_area.width,
            1,
        );
        frame.render_widget(quit_paragraph, quit_rect);

        // Render dialog if open
        if self.dialog_open {
            self.render_dialog(frame, primary_color, text_color, background_color);
        }
    }

    /// Renders the API endpoint dialog in the center of the screen
    fn render_dialog(
        &self,
        frame: &mut Frame,
        primary_color: Color,
        text_color: Color,
        background_color: Color,
    ) {
        let area = frame.area();

        // Calculate dialog size and position (centered)
        let dialog_width = 50.min(area.width - 4);
        let dialog_height = 5;
        let x = (area.width - dialog_width) / 2;
        let y = (area.height - dialog_height) / 2;

        let dialog_rect = Rect::new(x, y, dialog_width, dialog_height);

        // Clear the area behind the dialog
        frame.render_widget(Clear, dialog_rect);

        // Create dialog block with border
        let dialog_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(primary_color))
            .style(Style::default().bg(background_color));

        // Calculate inner area before rendering the block
        let inner_area = dialog_block.inner(dialog_rect);

        frame.render_widget(dialog_block, dialog_rect);

        // Split into label and input areas
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Label
                Constraint::Length(1), // Input
            ])
            .split(inner_area);

        // Render label
        let label = Paragraph::new("API endpoint name:").style(Style::default().fg(text_color));
        frame.render_widget(label, chunks[0]);

        // Render input field
        let input_text = if self.api_endpoint_input.is_empty() {
            "_".to_string()
        } else {
            format!("{}_", self.api_endpoint_input)
        };

        let input = Paragraph::new(input_text).style(Style::default().fg(primary_color));
        frame.render_widget(input, chunks[1]);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    fn handle_crossterm_events(&mut self) -> Result<(), RextTuiError> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn on_key_event(&mut self, key: KeyEvent) {
        if self.dialog_open {
            // Handle dialog input
            match key.code {
                KeyCode::Enter => {
                    // Close dialog and process the API endpoint name
                    let api_endpoint_name = self.api_endpoint_input.clone();
                    self.close_dialog();
                    self.handle_api_endpoint_creation(api_endpoint_name);
                }
                KeyCode::Esc => {
                    self.close_dialog();
                }
                KeyCode::Backspace => {
                    self.api_endpoint_input.pop();
                }
                KeyCode::Char(c) => {
                    self.api_endpoint_input.push(c);
                }
                _ => {}
            }
        } else {
            // Handle main app input
            match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
                (_, KeyCode::Char('e') | KeyCode::Char('E')) => self.open_dialog(),
                _ => {}
            }
        }
    }

    /// Opens the API endpoint creation dialog
    fn open_dialog(&mut self) {
        self.dialog_open = true;
        self.api_endpoint_input.clear();
    }

    /// Closes the API endpoint creation dialog
    fn close_dialog(&mut self) {
        self.dialog_open = false;
        self.api_endpoint_input.clear();
    }

    /// Handles API endpoint creation - placeholder for future functionality
    fn handle_api_endpoint_creation(&self, api_endpoint_name: String) -> String {
        // For now, just return the API endpoint name
        // Later this will interface with rext-core functionality
        api_endpoint_name
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    /// Loads the color configs, falling back to defaults if loading fails
    fn load_colors(&self) -> (Color, Color, Color) {
        // Try to load color configs, fall back to defaults on error
        match load_config("config/rext_tui.toml") {
            Ok(config) => {
                let primary_color = Color::Rgb(
                    config.default_colors.primary.r,
                    config.default_colors.primary.g,
                    config.default_colors.primary.b,
                );
                let text_color = Color::Rgb(
                    config.default_colors.text.r,
                    config.default_colors.text.g,
                    config.default_colors.text.b,
                );
                let background_color = Color::Rgb(
                    config.default_colors.background.r,
                    config.default_colors.background.g,
                    config.default_colors.background.b,
                );
                (primary_color, text_color, background_color)
            }
            Err(_) => {
                // Fall back to default colors
                let primary_color = Color::Rgb(255, 107, 53); // #ff6b35
                let text_color = Color::Rgb(204, 204, 204); // #cccccc
                let background_color = Color::Rgb(26, 26, 26); // #1a1a1a
                (primary_color, text_color, background_color)
            }
        }
    }
}
