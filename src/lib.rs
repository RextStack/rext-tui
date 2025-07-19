//! # Rext TUI - Terminal User Interface for Rext Development
//!
//! A TUI development tool for scaffolding and managing Rext fullstack web applications.
//!
//! ## Features
//!
//! - **Localization**: Multi-language support with comprehensive key binding system
//! - **Theme System**: Multiple color themes with easy switching
//!
//! ## Configuration
//!
//! The TUI uses TOML configuration files for themes, localization, and user preferences.
//! See the [`config`] module for detailed information about configuration file formats
//! and locations.
//!
//! ## Localization
//!
//! Full localization support with the [`localization`] module for text and key bindings.
//!
//! ## TODO
//!
//! - The render and app loop should not fail due to missing or failed config files and loads.
//! - Update the app so we have sensible defaults when any config files are missing or fail to load.

pub mod config;
pub mod error;
pub mod localization;

use crate::config::{
    get_available_languages_with_display, get_available_themes, load_current_language,
    load_current_theme, load_theme_colors, save_current_language, save_current_theme,
};
use crate::error::RextTuiError;
use crate::localization::Localization;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

/// Dialog types for the application
///
/// - `None`: No dialog is open, the main app is running
/// - `ApiEndpoint`: API endpoint creation dialog
/// - `Settings`: Settings dialog
/// - `Language`: Language selection dialog
#[derive(Debug, Clone, PartialEq)]
pub enum DialogType {
    None,
    ApiEndpoint,
    Settings,
    Language,
}

/// Settings dialog options
///
/// - `Theme`: Theme selection
/// - `Language`: Language selection
/// - `Close`: Close the dialog
#[derive(Debug, Clone, PartialEq)]
pub enum SettingsOption {
    Theme,
    Language,
    Close,
}

/// The main application which holds the state and logic of the application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Current active dialog
    pub current_dialog: DialogType,
    /// Text input buffer for API endpoint name
    pub api_endpoint_input: String,
    /// Current theme name
    pub current_theme: String,
    /// Localization system
    pub localization: Localization,
    /// Settings dialog selected index
    pub settings_selected: usize,
    /// Language dialog search input
    pub language_search: String,
    /// Language dialog selected index
    pub language_selected: usize,
    /// Filtered languages list
    pub filtered_languages: Vec<(String, String)>,
    /// Language dialog list state
    pub language_list_state: ListState,
}

/// Theme colors
///
/// - `primary`: Accent color for highlights, borders, and interactive elements
/// - `text`: Regular text color for most content
/// - `background`: Background color for the entire application
struct Theme {
    primary: Color,
    text: Color,
    background: Color,
}

/// Macro for creating ratatui styled spans with localization and color
#[macro_export]
macro_rules! styled_span {
    // Create a styled span with localization and color
    ($localization:expr, $method:ident, $key:expr, $color:expr) => {
        ratatui::text::Span::styled(
            $localization.$method($key),
            ratatui::style::Style::default().fg($color),
        )
    };
    // Create a styled span with localization, color, and additional style modifiers
    ($localization:expr, $method:ident, $key:expr, $color:expr, $($modifier:ident),+) => {
        ratatui::text::Span::styled(
            $localization.$method($key),
            ratatui::style::Style::default().fg($color)$(.$modifier())+,
        )
    };
}

/// Macro for creating ratatui Line objects with multiple styled spans
#[macro_export]
macro_rules! styled_line {
    // Create a line with multiple styled spans
    ($($localization:expr, $method:ident, $key:expr, $color:expr $(, $($modifier:ident),+)?);+ $(;)?) => {
        ratatui::text::Line::from(vec![
            $(
                styled_span!($localization, $method, $key, $color $(, $($modifier),+)?),
            )+
        ])
    };
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
            current_dialog: DialogType::None,
            api_endpoint_input: String::new(),
            current_theme: "rust".to_string(), // rust is the default theme
            localization,
            settings_selected: 0,
            language_search: String::new(),
            language_selected: 0,
            filtered_languages: Vec::new(),
            language_list_state: ListState::default(),
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
            current_dialog: DialogType::None,
            api_endpoint_input: String::new(),
            current_theme,
            localization,
            settings_selected: 0,
            language_search: String::new(),
            language_selected: 0,
            filtered_languages: Vec::new(),
            language_list_state: ListState::default(),
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
    /// This is responsible for setting the theme, localizations, and drawing the main app screen
    fn render(&mut self, frame: &mut Frame) {
        // Load colors
        let (primary_color, text_color, background_color) = self.load_colors();
        let theme = Theme {
            primary: primary_color,
            text: text_color,
            background: background_color,
        };

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
                Constraint::Length(30), // Right side for settings button
            ])
            .split(top_area);

        // Left side: "add API endpoint" button
        let button_text = styled_line!(
            self.localization, ui, "add_api_endpoint", primary_color, bold;
            self.localization, ui, "add_api_endpoint_shortcut", text_color
        );

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

        // Right side: settings button
        let settings_text = styled_line!(
            self.localization, ui, "settings_title", primary_color, bold;
            self.localization, ui, "settings_shortcut", text_color
        );

        let settings_paragraph = Paragraph::new(settings_text)
            .style(Style::default().fg(text_color))
            .alignment(Alignment::Right);
        frame.render_widget(
            settings_paragraph,
            Rect::new(
                top_chunks[1].x,
                top_chunks[1].y + 1,
                top_chunks[1].width - 1,
                1,
            ),
        );

        // Bottom area with quit instructions
        let bottom_area = chunks[1];
        let quit_instructions = styled_line!(
            self.localization, msg, "quit_instruction_prefix", text_color;
            self.localization, key, "quit", primary_color, bold;
            self.localization, msg, "quit_instruction_middle", text_color;
            self.localization, key, "quit_combo", primary_color, bold;
            self.localization, msg, "quit_instruction_suffix", text_color
        );

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
        if self.current_dialog != DialogType::None {
            self.render_dialog(frame, theme);
        }
    }

    /// Renders the appropriate dialog based on current_dialog type, via the DialogType enum
    fn render_dialog(&mut self, frame: &mut Frame, theme: Theme) {
        match &self.current_dialog {
            DialogType::ApiEndpoint => self.render_api_endpoint_dialog(frame, theme),
            DialogType::Settings => self.render_settings_dialog(frame, theme),
            DialogType::Language => self.render_language_dialog(frame, theme),
            DialogType::None => {}
        }
    }

    /// Renders the API endpoint dialog in the center of the screen
    ///
    /// - `frame`: The frame to render the dialog on
    /// - `t`: The theme to use for the dialog
    ///
    /// > This dialog will be used to create a new API endpoint in a Rext app- does nothing right now.
    /// > **WARNING**: This is a stub, needs to call the rext-core functions to create the API endpoint. TBD.
    fn render_api_endpoint_dialog(&self, frame: &mut Frame, t: Theme) {
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
            .border_style(Style::default().fg(t.primary))
            .style(Style::default().bg(t.background));

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
            .style(Style::default().fg(t.text));
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

        let input = Paragraph::new(input_text).style(Style::default().fg(t.primary));
        frame.render_widget(input, chunks[1]);
    }

    /// Renders the settings dialog
    ///
    /// - `frame`: The frame to render the dialog on
    /// - `t`: The theme to use for the dialog
    ///
    /// This dialog displays a list of settings: theme and language selection, with a close option.
    fn render_settings_dialog(&self, frame: &mut Frame, t: Theme) {
        let area = frame.area();

        // Calculate dialog size and position (centered)
        let dialog_width = 60.min(area.width - 4);
        let dialog_height = 8;
        let x = (area.width - dialog_width) / 2;
        let y = (area.height - dialog_height) / 2;

        let dialog_rect = Rect::new(x, y, dialog_width, dialog_height);

        // Clear the area behind the dialog
        frame.render_widget(Clear, dialog_rect);

        // Create dialog block with border
        let dialog_block = Block::default()
            .title(self.localization.ui("settings_title"))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(t.primary))
            .style(Style::default().bg(t.background));

        let inner_area = dialog_block.inner(dialog_rect);
        frame.render_widget(dialog_block, dialog_rect);

        // Settings options
        let settings_options = vec![
            format!(
                "{}: {}",
                self.localization.ui("theme_setting"),
                self.current_theme
            ),
            self.localization.ui("language_setting").to_string(),
            self.localization.ui("close_dialog").to_string(),
        ];

        let items: Vec<ListItem> = settings_options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == self.settings_selected {
                    Style::default().fg(t.primary).bold()
                } else {
                    Style::default().fg(t.text)
                };
                ListItem::new(option.clone()).style(style)
            })
            .collect();

        let list = List::new(items);
        frame.render_widget(list, inner_area);

        // Render instruction at the bottom
        let instruction_rect = Rect::new(
            dialog_rect.x + 1,
            dialog_rect.y + dialog_rect.height,
            dialog_rect.width - 2,
            1,
        );
        let instruction = Paragraph::new(self.localization.msg("settings_instruction"))
            .style(Style::default().fg(t.text));
        frame.render_widget(instruction, instruction_rect);
    }

    /// Renders the language selection dialog
    ///
    /// - `frame`: The frame to render the dialog on
    /// - `t`: The theme to use for the dialog
    ///
    /// This dialog displays a list of languages, with a search box and a list of languages.
    fn render_language_dialog(&mut self, frame: &mut Frame, t: Theme) {
        let area = frame.area();

        // Calculate dialog size and position (centered)
        let dialog_width = 60.min(area.width - 4);
        let dialog_height = 15.min(area.height - 4);
        let x = (area.width - dialog_width) / 2;
        let y = (area.height - dialog_height) / 2;

        let dialog_rect = Rect::new(x, y, dialog_width, dialog_height);

        // Clear the area behind the dialog
        frame.render_widget(Clear, dialog_rect);

        // Create dialog block with border
        let dialog_block = Block::default()
            .title(self.localization.ui("language_dialog_title"))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(t.primary))
            .style(Style::default().bg(t.background));

        let inner_area = dialog_block.inner(dialog_rect);
        frame.render_widget(dialog_block, dialog_rect);

        // Split into search box and list
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Search box
                Constraint::Min(0),    // Language list
            ])
            .split(inner_area);

        // Render search box
        let search_text = if self.language_search.is_empty() {
            self.localization
                .ui("language_search_placeholder")
                .to_string()
        } else {
            format!(
                "{}{}",
                self.language_search,
                self.localization.ui("input_cursor")
            )
        };

        let search_box = Paragraph::new(search_text)
            .style(Style::default().fg(t.primary))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(t.text)),
            );
        frame.render_widget(search_box, chunks[0]);

        // Render language list
        if self.filtered_languages.is_empty() {
            let no_results = Paragraph::new(self.localization.ui("no_languages_found"))
                .style(Style::default().fg(t.text))
                .alignment(Alignment::Center);
            frame.render_widget(no_results, chunks[1]);
        } else {
            let items: Vec<ListItem> = self
                .filtered_languages
                .iter()
                .enumerate()
                .map(|(i, (_, display))| {
                    let style = if i == self.language_selected {
                        Style::default().fg(t.primary).bold()
                    } else {
                        Style::default().fg(t.text)
                    };
                    ListItem::new(display.clone()).style(style)
                })
                .collect();

            let list = List::new(items);
            self.language_list_state
                .select(Some(self.language_selected));
            frame.render_stateful_widget(list, chunks[1], &mut self.language_list_state);
        }

        // Render instruction at the bottom
        let instruction_rect = Rect::new(
            dialog_rect.x + 1,
            dialog_rect.y + dialog_rect.height,
            dialog_rect.width - 2,
            1,
        );
        let instruction = Paragraph::new(self.localization.msg("language_instruction"))
            .style(Style::default().fg(t.text));
        frame.render_widget(instruction, instruction_rect);
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
        match &self.current_dialog {
            DialogType::ApiEndpoint => {
                self.handle_api_endpoint_dialog_events(key);
            }
            DialogType::Settings => {
                self.handle_settings_dialog_events(key);
            }
            DialogType::Language => {
                self.handle_language_dialog_events(key);
            }
            DialogType::None => {
                self.handle_main_app_events(key);
            }
        }
    }

    /// Handles events for the API endpoint dialog
    fn handle_api_endpoint_dialog_events(&mut self, key: KeyEvent) {
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
    }

    /// Handles events for the settings dialog
    fn handle_settings_dialog_events(&mut self, key: KeyEvent) {
        if self
            .localization
            .matches_key("escape", key.modifiers, key.code)
        {
            self.close_dialog();
        } else if self.localization.matches_key("up", key.modifiers, key.code) {
            if self.settings_selected > 0 {
                self.settings_selected -= 1;
            } else {
                self.settings_selected = 2; // Wrap to bottom (Close option)
            }
        } else if self
            .localization
            .matches_key("down", key.modifiers, key.code)
        {
            self.settings_selected = (self.settings_selected + 1) % 3;
        } else if self
            .localization
            .matches_key("enter", key.modifiers, key.code)
        {
            match self.settings_selected {
                0 => {
                    // Theme option
                    self.cycle_theme();
                }
                1 => {
                    // Language option
                    self.open_language_dialog();
                }
                2 => {
                    // Close option
                    self.close_dialog();
                }
                _ => {}
            }
        }
    }

    /// Handles events for the language dialog
    fn handle_language_dialog_events(&mut self, key: KeyEvent) {
        if self
            .localization
            .matches_key("escape", key.modifiers, key.code)
        {
            self.close_dialog();
        } else if self.localization.matches_key("up", key.modifiers, key.code) {
            if !self.filtered_languages.is_empty() && self.language_selected > 0 {
                self.language_selected -= 1;
            } else if !self.filtered_languages.is_empty() {
                self.language_selected = self.filtered_languages.len() - 1;
            }
        } else if self
            .localization
            .matches_key("down", key.modifiers, key.code)
        {
            if !self.filtered_languages.is_empty() {
                self.language_selected =
                    (self.language_selected + 1) % self.filtered_languages.len();
            }
        } else if self
            .localization
            .matches_key("enter", key.modifiers, key.code)
        {
            if !self.filtered_languages.is_empty() {
                let selected_language = self.filtered_languages[self.language_selected].0.clone();
                self.select_language(selected_language);
            }
        } else if self
            .localization
            .matches_key("backspace", key.modifiers, key.code)
        {
            self.language_search.pop();
            self.filter_languages();
        } else if let KeyCode::Char(c) = key.code {
            self.language_search.push(c);
            self.filter_languages();
        }
    }

    /// Handles events for the main application
    fn handle_main_app_events(&mut self, key: KeyEvent) {
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
            self.open_dialog(DialogType::ApiEndpoint);
        } else if self
            .localization
            .matches_key("settings", key.modifiers, key.code)
        {
            self.open_dialog(DialogType::Settings);
        }
    }

    /// Opens the API endpoint creation dialog
    fn open_dialog(&mut self, dialog_type: DialogType) {
        self.current_dialog = dialog_type;
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

    /// Opens the language selection dialog
    fn open_language_dialog(&mut self) {
        self.current_dialog = DialogType::Language;
        self.language_search.clear();
        self.language_selected = 0;
        self.filter_languages();
    }

    /// Selects a language and closes the dialog
    fn select_language(&mut self, language_code: String) {
        // Save the selected language to config
        if let Err(_) = save_current_language(&language_code) {
            // Handle error gracefully - in production, you might want to show an error message
            return;
        }

        // Reload the localization with the new language
        if let Err(_) = self.localization.reload(&language_code) {
            // Handle error gracefully - fallback to English if reload fails
            let _ = self.localization.reload("en");
        }

        self.close_dialog();
    }

    /// Filters the languages based on the search input
    fn filter_languages(&mut self) {
        let search_term = self.language_search.to_lowercase();

        if let Ok(available_languages) = get_available_languages_with_display() {
            self.filtered_languages = available_languages
                .into_iter()
                .filter(|(code, display)| {
                    code.to_lowercase().contains(&search_term)
                        || display.to_lowercase().contains(&search_term)
                })
                .collect();
        } else {
            self.filtered_languages = Vec::new();
        }

        // Reset selected index, ensuring it's within bounds
        self.language_selected = 0;
        if !self.filtered_languages.is_empty()
            && self.language_selected >= self.filtered_languages.len()
        {
            self.language_selected = self.filtered_languages.len() - 1;
        }

        // If only one item after filtering, we could auto-select it on Enter
        // The current implementation allows navigation even with one item
    }

    /// Closes the current dialog and resets dialog-specific state
    fn close_dialog(&mut self) {
        self.current_dialog = DialogType::None;
        self.api_endpoint_input.clear();
        self.language_search.clear();
        self.language_selected = 0;
        self.settings_selected = 0;
        self.filtered_languages.clear();
    }
}
