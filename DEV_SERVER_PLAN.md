# Development Server Implementation Plan

## Project Overview

This document outlines the implementation plan for adding a development server command to rext-tui that manages both frontend (Node.js/npm/Vite) and backend (Rust/Cargo) development servers concurrently, with organized log output in the TUI.

## Proposed Architecture Analysis

### Approach Summary
- **Core Logic Location**: All development server logic in rext-core
- **Process Management**: Run both `npm run dev` and `cargo run` concurrently using async task runners
- **Output Handling**: Collect stdout/stderr from both processes in rext-core
- **Communication**: Send output streams from rext-core to rext-tui
- **UI Layout**: Two-panel TUI display (top: frontend logs, bottom: backend logs)

## Pros and Cons Analysis

### ✅ Pros

1. **Separation of Concerns**
   - Maintains clean architecture with business logic in core, UI logic in TUI
   - Follows existing microkernel pattern of rext-core
   - Allows future frontends (CLI, web interface) to reuse dev server functionality

2. **Centralized Process Management**
   - Single source of truth for development server state
   - Easier to implement process lifecycle management (start, stop, restart, health checks)
   - Better error handling and recovery strategies

3. **Consistent User Experience**
   - Organized, split-pane log viewing prevents log interleaving confusion
   - Color-coded output per service for better readability
   - Leverages existing TUI theming and localization systems

4. **Extensibility**
   - Easy to add more development services (database, Redis, etc.)
   - Can add features like log filtering, search, and export
   - Foundation for advanced dev tooling (hot reload notifications, build status)

5. **Resource Management**
   - Proper cleanup when TUI exits
   - Can implement resource usage monitoring
   - Better handling of port conflicts and service dependencies

### ❌ Cons

1. **Complexity**
   - Increases codebase complexity with async process management
   - Inter-process communication adds potential failure points
   - More complex error handling across process boundaries

2. **Performance Overhead**
   - Additional memory usage for buffering output streams
   - Network/IPC overhead for streaming logs to TUI
   - Potential latency in log display vs direct terminal output

3. **Development Dependencies**
   - Requires both npm and cargo to be available in PATH
   - Assumes specific project structure (package.json, Cargo.toml)
   - May need version compatibility checks

4. **Platform Compatibility**
   - Process spawning behavior differs across platforms
   - Signal handling varies between Windows/Unix systems
   - Terminal capabilities may affect display quality

5. **Debugging Complexity**
   - Harder to debug individual services when wrapped
   - Log timestamps and formatting may be altered
   - Potential loss of interactive features (like npm prompts)

## Implementation Plan

### Phase 1: Core Infrastructure (rext-core)

#### 1.1 Error Types Extension
```rust
// Add to rext-core/src/error.rs
#[derive(thiserror::Error, Debug)]
pub enum RextCoreError {
    // ... existing variants ...

    #[error("Development server failed to start: {0}")]
    DevServerStart(String),

    #[error("Process spawn failed: {0}")]
    ProcessSpawn(#[from] std::process::Error),

    #[error("Stream communication error: {0}")]
    StreamError(String),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Port conflict on {0}")]
    PortConflict(u16),
}
```

#### 1.2 Development Server Core Module
```rust
// Create rext-core/src/dev_server.rs

use tokio::process::{Child, Command};
use tokio::sync::mpsc;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::collections::HashMap;

pub struct DevServerManager {
    services: HashMap<String, ServiceInfo>,
    log_sender: mpsc::UnboundedSender<LogMessage>,
}

pub struct ServiceInfo {
    process: Option<Child>,
    status: ServiceStatus,
    port: Option<u16>,
    start_time: std::time::Instant,
}

#[derive(Debug, Clone)]
pub enum ServiceStatus {
    Starting,
    Running,
    Failed(String),
    Stopped,
}

#[derive(Debug, Clone)]
pub struct LogMessage {
    pub service: String,
    pub timestamp: std::time::SystemTime,
    pub level: LogLevel,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
```

#### 1.3 Service Detection and Configuration
```rust
// Service configuration and auto-detection
pub struct DevServerConfig {
    pub frontend: Option<FrontendConfig>,
    pub backend: Option<BackendConfig>,
    pub services: Vec<CustomService>,
}

pub struct FrontendConfig {
    pub command: String,        // e.g., "npm"
    pub args: Vec<String>,      // e.g., ["run", "dev"]
    pub working_dir: PathBuf,
    pub port: Option<u16>,
}

pub struct BackendConfig {
    pub command: String,        // e.g., "cargo"
    pub args: Vec<String>,      // e.g., ["run"]
    pub working_dir: PathBuf,
    pub port: Option<u16>,
}

impl DevServerConfig {
    pub fn auto_detect() -> Result<Self, RextCoreError> {
        // Auto-detect frontend: package.json, vite.config.*, etc.
        // Auto-detect backend: Cargo.toml, main.rs, etc.
        // Read ports from config files when possible
    }
}
```

#### 1.4 Process Management
```rust
impl DevServerManager {
    pub async fn start_service(&mut self, name: &str, config: &ServiceConfig) -> Result<(), RextCoreError> {
        // Spawn process with proper stdio handling
        // Set up log streaming
        // Update service status
    }

    pub async fn stop_service(&mut self, name: &str) -> Result<(), RextCoreError> {
        // Graceful shutdown with timeout
        // Force kill if necessary
        // Cleanup resources
    }

    pub async fn restart_service(&mut self, name: &str) -> Result<(), RextCoreError> {
        // Stop then start service
        // Preserve configuration
    }

    pub fn get_service_status(&self, name: &str) -> Option<&ServiceStatus> {
        // Return current status
    }

    pub async fn shutdown_all(&mut self) -> Result<(), RextCoreError> {
        // Gracefully shutdown all services
        // Wait for cleanup with timeout
    }
}
```

#### 1.5 Log Streaming System
```rust
pub struct LogStreamer {
    receiver: mpsc::UnboundedReceiver<LogMessage>,
    buffer: VecDeque<LogMessage>,
    max_buffer_size: usize,
}

impl LogStreamer {
    pub fn new(receiver: mpsc::UnboundedReceiver<LogMessage>) -> Self {
        Self {
            receiver,
            buffer: VecDeque::with_capacity(1000),
            max_buffer_size: 10000,
        }
    }

    pub async fn get_recent_logs(&mut self, service: Option<&str>, count: usize) -> Vec<LogMessage> {
        // Drain new messages from receiver
        // Filter by service if specified
        // Return most recent messages
    }

    pub async fn wait_for_logs(&mut self, timeout: Duration) -> Vec<LogMessage> {
        // Wait for new logs with timeout
        // Return any new messages received
    }
}
```

#### 1.6 Public API
```rust
// Add to rext-core/src/lib.rs

pub use dev_server::{DevServerManager, DevServerConfig, LogMessage, LogLevel, ServiceStatus};

pub async fn start_dev_servers() -> Result<(DevServerManager, LogStreamer), RextCoreError> {
    let config = DevServerConfig::auto_detect()?;
    let (tx, rx) = mpsc::unbounded_channel();

    let mut manager = DevServerManager::new(tx);
    let streamer = LogStreamer::new(rx);

    // Start detected services
    if let Some(frontend) = config.frontend {
        manager.start_service("frontend", &frontend.into()).await?;
    }

    if let Some(backend) = config.backend {
        manager.start_service("backend", &backend.into()).await?;
    }

    Ok((manager, streamer))
}

pub fn check_dev_dependencies() -> Result<Vec<String>, RextCoreError> {
    // Check for npm, cargo, node, rust toolchain
    // Return list of missing dependencies
}
```

### Phase 2: TUI Integration (rext-tui)

#### 2.1 New Dialog Type
```rust
// Add to rext-tui/src/lib.rs DialogType enum
pub enum DialogType {
    // ... existing variants ...
    DevServer,
}
```

#### 2.2 Dev Server State Management
```rust
// Add to App struct
pub struct App {
    // ... existing fields ...

    /// Development server manager
    pub dev_server_manager: Option<DevServerManager>,
    /// Log streamer for receiving dev server logs
    pub log_streamer: Option<LogStreamer>,
    /// Frontend logs buffer
    pub frontend_logs: VecDeque<LogMessage>,
    /// Backend logs buffer
    pub backend_logs: VecDeque<LogMessage>,
    /// Selected log panel (0 = frontend, 1 = backend)
    pub selected_log_panel: usize,
    /// Log scroll position for each panel
    pub log_scroll_positions: [usize; 2],
    /// Auto-scroll enabled for each panel
    pub auto_scroll: [bool; 2],
    /// Dev server dialog selected option
    pub dev_server_selected: usize,
}
```

#### 2.3 Dev Server Dialog Rendering
```rust
impl App {
    fn render_dev_server_dialog(&mut self, frame: &mut Frame, theme: Theme) {
        // Split screen into two panels
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header with service status
                Constraint::Min(0),     // Log panels
                Constraint::Length(3),  // Controls/instructions
            ])
            .split(dialog_rect);

        // Render service status header
        self.render_service_status_header(frame, main_chunks[0], theme);

        // Split log area into frontend/backend panels
        let log_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), // Frontend logs
                Constraint::Percentage(50), // Backend logs
            ])
            .split(main_chunks[1]);

        // Render log panels
        self.render_log_panel(frame, log_chunks[0], "Frontend", &self.frontend_logs, 0, theme);
        self.render_log_panel(frame, log_chunks[1], "Backend", &self.backend_logs, 1, theme);

        // Render controls
        self.render_dev_server_controls(frame, main_chunks[2], theme);
    }

    fn render_log_panel(&self, frame: &mut Frame, area: Rect, title: &str, logs: &VecDeque<LogMessage>, panel_idx: usize, theme: Theme) {
        let is_selected = self.selected_log_panel == panel_idx;
        let border_style = if is_selected {
            Style::default().fg(theme.primary)
        } else {
            Style::default().fg(theme.text)
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        // Render logs with color coding
        let log_items: Vec<ListItem> = logs
            .iter()
            .skip(self.log_scroll_positions[panel_idx])
            .take(inner_area.height as usize)
            .map(|log| {
                let style = match log.level {
                    LogLevel::Error => Style::default().fg(Color::Red),
                    LogLevel::Warning => Style::default().fg(Color::Yellow),
                    LogLevel::Info => Style::default().fg(theme.text),
                    LogLevel::Debug => Style::default().fg(Color::Gray),
                };

                let timestamp = format_timestamp(log.timestamp);
                let content = format!("[{}] {}", timestamp, log.content);
                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(log_items);
        frame.render_widget(list, inner_area);
    }
}
```

#### 2.4 Event Handling
```rust
impl App {
    fn handle_dev_server_dialog_events(&mut self, key: KeyEvent) {
        // Tab to switch between panels
        if self.localization.matches_key("tab", key.modifiers, key.code) {
            self.selected_log_panel = (self.selected_log_panel + 1) % 2;
        }
        // Scroll controls
        else if self.localization.matches_key("up", key.modifiers, key.code) {
            let pos = &mut self.log_scroll_positions[self.selected_log_panel];
            *pos = pos.saturating_sub(1);
            self.auto_scroll[self.selected_log_panel] = false;
        }
        else if self.localization.matches_key("down", key.modifiers, key.code) {
            self.log_scroll_positions[self.selected_log_panel] += 1;
            self.auto_scroll[self.selected_log_panel] = false;
        }
        // Page up/down
        else if self.localization.matches_key("page_up", key.modifiers, key.code) {
            let pos = &mut self.log_scroll_positions[self.selected_log_panel];
            *pos = pos.saturating_sub(10);
            self.auto_scroll[self.selected_log_panel] = false;
        }
        // ... additional controls
        // R to restart service
        // S to stop service
        // C to clear logs
        // A to toggle auto-scroll
        // Escape to close
    }

    async fn update_dev_server_logs(&mut self) -> Result<(), RextTuiError> {
        if let Some(streamer) = &mut self.log_streamer {
            let new_logs = streamer.wait_for_logs(Duration::from_millis(100)).await;

            for log in new_logs {
                match log.service.as_str() {
                    "frontend" => {
                        self.frontend_logs.push_back(log);
                        if self.frontend_logs.len() > 1000 {
                            self.frontend_logs.pop_front();
                        }
                        // Auto-scroll if enabled
                        if self.auto_scroll[0] {
                            self.log_scroll_positions[0] = self.frontend_logs.len().saturating_sub(20);
                        }
                    }
                    "backend" => {
                        self.backend_logs.push_back(log);
                        if self.backend_logs.len() > 1000 {
                            self.backend_logs.pop_front();
                        }
                        if self.auto_scroll[1] {
                            self.log_scroll_positions[1] = self.backend_logs.len().saturating_sub(20);
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
```

#### 2.5 Localization Updates
```toml
# Add to rext-tui/localization/en.toml

[dev_server]
title = "Development Servers"
frontend_panel = "Frontend (npm run dev)"
backend_panel = "Backend (cargo run)"
starting = "Starting..."
running = "Running"
failed = "Failed"
stopped = "Stopped"

[dev_server_controls]
tab_switch = "Tab"
scroll = "↑↓"
page_scroll = "PgUp/PgDn"
restart = "R"
stop = "S"
clear = "C"
auto_scroll = "A"
close = "Esc"

[dev_server_instructions]
tab_switch = "Switch panels"
scroll = "Scroll logs"
restart_service = "Restart service"
stop_service = "Stop service"
clear_logs = "Clear logs"
toggle_autoscroll = "Toggle auto-scroll"
close_dialog = "Close"
```

### Phase 3: Testing and Validation

#### 3.1 Unit Tests
- Test process spawning and management
- Test log parsing and filtering
- Test error handling scenarios
- Test configuration detection

#### 3.2 Integration Tests
- Test full dev server lifecycle
- Test TUI log display and scrolling
- Test service restart and cleanup
- Test platform compatibility

#### 3.3 Error Scenarios
- Missing dependencies (npm, cargo)
- Port conflicts
- Invalid project structure
- Process crashes and recovery

### Phase 4: Advanced Features (Future)

#### 4.1 Service Health Monitoring
- HTTP health check endpoints
- Process resource usage monitoring
- Automatic restart on crashes

#### 4.2 Log Enhancements
- Log filtering and search
- Export logs to files
- Real-time log statistics

#### 4.3 Build Integration
- Hot reload detection
- Build status notifications
- Asset compilation status

#### 4.4 Multi-Project Support
- Support for monorepos
- Service dependency graphs
- Coordinated startup sequences

## Technical Considerations

### Memory Management
- Implement log rotation to prevent memory leaks
- Use bounded channels for backpressure
- Monitor resource usage in long-running sessions

### Performance Optimization
- Lazy log rendering for better scroll performance
- Efficient log filtering algorithms
- Minimize allocations in hot paths

### Error Recovery
- Graceful degradation when services fail
- Automatic retry mechanisms with backoff
- Clear error reporting to users

### Platform Support
- Handle Windows vs Unix process differences
- Account for different terminal capabilities
- Test on multiple OS/terminal combinations

## Success Metrics

1. **Functionality**: Successfully runs and displays logs from both development servers
2. **Usability**: Intuitive navigation and log viewing experience
3. **Reliability**: Handles service crashes and restarts gracefully
4. **Performance**: Responsive UI even with high log volume
5. **Maintainability**: Clean, well-documented code following project conventions

## Conclusion

This implementation plan provides a solid foundation for adding development server management to rext-tui while maintaining the project's architectural principles. The approach balances functionality with complexity, providing a powerful development tool while keeping the codebase maintainable and extensible.

The modular design allows for incremental development and testing, with clear interfaces between components. Future enhancements can be added without major architectural changes, ensuring long-term sustainability of the feature.