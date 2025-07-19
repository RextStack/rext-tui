//! The main entry point for the TUI
//!
//! TODO - The render and app loop should not fail due to missing or failed config files and loads.
//! Update the app so we have sensible defaults when any config files are missing or fail to load.
pub mod config;
pub mod error;
pub mod localization;

use crate::config::{
    get_available_themes, load_current_language, load_current_theme, load_theme_colors,
    save_current_theme,
};
use crate::error::RextTuiError;
use crate::localization::Localization;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

/// The main application which holds the state and logic of the application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Is the API endpoint dialog open?
    pub dialog_open: bool,
    /// Text input buffer for API endpoint name
    pub api_endpoint_input: String,
    /// Current theme name
    pub current_theme: String,
    /// Localization system
    pub localization: Localization,
}

impl Default for App {
    fn default() -> Self {
        // get the language from the current_localization.toml file
        let language = load_current_language().unwrap_or_else(|_| "en".to_string());
        let localization = Localization::new(&language).unwrap_or_else(|_| {
            Localization::new("en").expect("Failed to load English localization")
        });

        Self {
            running: false,
            dialog_open: false,
            api_endpoint_input: String::new(),
            current_theme: "rust".to_string(), // rust is the default theme
            localization,
        }
    }
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let current_theme = load_current_theme().unwrap_or_else(|_| "rust".to_string());
        let language = load_current_language().unwrap_or_else(|_| "en".to_string());
        let localization = Localization::new(&language).unwrap_or_else(|_| {
            // If we can't load localization, create a minimal fallback
            // This shouldn't happen in normal operation since we ship with en.toml
            Localization::new("en").expect("Failed to load English localization")
        });

        Self {
            running: false,
            dialog_open: false,
            api_endpoint_input: String::new(),
            current_theme,
            localization,
        }
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

        // Top area with buttons
        let top_area = chunks[0];

        // Split top area into left and right sections
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),     // Left side for API endpoint button
                Constraint::Length(30), // Right side for theme button
            ])
            .split(top_area);

        // Left side: "add API endpoint" button
        let button_text = Line::from(vec![
            Span::styled(
                self.localization.ui("add_api_endpoint"),
                Style::default().fg(primary_color).bold(),
            ),
            Span::styled(
                self.localization.ui("add_api_endpoint_shortcut"),
                Style::default().fg(text_color),
            ),
        ]);

        let button_paragraph = Paragraph::new(button_text).style(Style::default().fg(text_color));
        frame.render_widget(
            button_paragraph,
            Rect::new(
                top_chunks[0].x + 1,
                top_chunks[0].y + 1,
                top_chunks[0].width,
                1,
            ),
        );

        // Right side: theme button
        let theme_text = Line::from(vec![
            Span::styled(
                self.localization.ui("theme_label"),
                Style::default().fg(text_color),
            ),
            Span::styled(
                &self.current_theme,
                Style::default().fg(primary_color).bold(),
            ),
            Span::styled(
                self.localization.ui("theme_shortcut"),
                Style::default().fg(text_color),
            ),
        ]);

        let theme_paragraph = Paragraph::new(theme_text)
            .style(Style::default().fg(text_color))
            .alignment(Alignment::Right);
        frame.render_widget(
            theme_paragraph,
            Rect::new(
                top_chunks[1].x,
                top_chunks[1].y + 1,
                top_chunks[1].width - 1,
                1,
            ),
        );

        // Bottom area with quit instructions
        let bottom_area = chunks[1];
        let quit_instructions = Line::from(vec![
            Span::styled(
                self.localization.msg("quit_instruction_prefix"),
                Style::default().fg(text_color),
            ),
            Span::styled(
                self.localization.key("quit"),
                Style::default().fg(primary_color).bold(),
            ),
            Span::styled(
                self.localization.msg("quit_instruction_middle"),
                Style::default().fg(text_color),
            ),
            Span::styled(
                self.localization.key("quit_combo"),
                Style::default().fg(primary_color).bold(),
            ),
            Span::styled(
                self.localization.msg("quit_instruction_suffix"),
                Style::default().fg(text_color),
            ),
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
        let label = Paragraph::new(self.localization.ui("api_endpoint_name_prompt"))
            .style(Style::default().fg(text_color));
        frame.render_widget(label, chunks[0]);

        // Render input field
        let input_text = if self.api_endpoint_input.is_empty() {
            self.localization.ui("input_cursor").to_string()
        } else {
            format!(
                "{}{}",
                self.api_endpoint_input,
                self.localization.ui("input_cursor")
            )
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
            // Handle dialog input using localized key bindings
            if self
                .localization
                .matches_key("enter", key.modifiers, key.code)
            {
                // Close dialog and process the API endpoint name
                let api_endpoint_name = self.api_endpoint_input.clone();
                self.close_dialog();
                self.handle_api_endpoint_creation(api_endpoint_name);
            } else if self
                .localization
                .matches_key("escape", key.modifiers, key.code)
            {
                self.close_dialog();
            } else if self
                .localization
                .matches_key("backspace", key.modifiers, key.code)
            {
                self.api_endpoint_input.pop();
            } else if let KeyCode::Char(c) = key.code {
                self.api_endpoint_input.push(c);
            }
        } else {
            // Handle main app input using localized key bindings
            if self
                .localization
                .matches_key("quit", key.modifiers, key.code)
                || self
                    .localization
                    .matches_key("quit_combo", key.modifiers, key.code)
                || self
                    .localization
                    .matches_key("escape", key.modifiers, key.code)
            {
                self.quit();
            } else if self
                .localization
                .matches_key("add_endpoint", key.modifiers, key.code)
            {
                self.open_dialog();
            } else if self
                .localization
                .matches_key("toggle_theme", key.modifiers, key.code)
            {
                self.cycle_theme();
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

    /// Loads the color configs from the current theme, falling back to defaults if loading fails
    fn load_colors(&self) -> (Color, Color, Color) {
        // Try to load colors from the current theme, fall back to defaults on error
        match load_theme_colors(&self.current_theme) {
            Ok(colors) => {
                let primary_color =
                    Color::Rgb(colors.primary.r, colors.primary.g, colors.primary.b);
                let text_color = Color::Rgb(colors.text.r, colors.text.g, colors.text.b);
                let background_color = Color::Rgb(
                    colors.background.r,
                    colors.background.g,
                    colors.background.b,
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

    /// Cycles to the next available theme
    fn cycle_theme(&mut self) {
        if let Ok(themes) = get_available_themes() {
            if let Some(current_index) = themes.iter().position(|t| t == &self.current_theme) {
                let next_index = (current_index + 1) % themes.len();
                self.current_theme = themes[next_index].clone();

                // Save the new theme selection
                let _ = save_current_theme(&self.current_theme);
            }
        }
    }
}
